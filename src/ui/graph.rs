
use std::collections::VecDeque as Queue;
use std::vec::IntoIter as VecIntoIter;
use std::slice::Iter as SliceIter;
use std::ops::Index;
use std::mem;

#[derive(Debug, Clone)]
pub struct IndexGraph {
    vertices: Vec<Vertex>,
}

#[derive(Debug, Clone, Default)]
pub struct Vertex {
    in_degree: usize,
    out_degree: usize,
    pub in_edges: Vec<usize>,
    pub out_edges: Vec<usize>,
}

#[derive(Debug)]
pub struct IndexGraphBuilder<'g> {
    graph: &'g mut IndexGraph,
    index: usize
}

impl IndexGraphBuilder<'_> {
    /// Returns a reference to the stored graph
    pub fn as_graph(&self) -> &IndexGraph {
        self.graph
    }

    /// Returns a mutable reference to the stored graph
    pub fn as_mut_graph(&mut self) -> &mut IndexGraph {
        self.graph
    }

    /// Returns the stored index
    pub fn index(&self) -> usize {
        self.index
    }

    /// Add an edge from the stored index to the passed index
    ///
    /// This method does not check for duplicate edges.
    pub fn add_out_edge(&mut self, index: usize) {
        self.graph.add_edge(self.index, index)
    }

    /// Add an edge from the passed index to the stored index
    ///
    /// This method does not check for duplicate edges.
    pub fn add_in_edge(&mut self, index: usize) {
        self.graph.add_edge(index, self.index)
    }
}

impl IndexGraph {

    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    pub fn with_vertices(len: usize) -> Self {
        let mut vertices = Vec::with_capacity(len);
        vertices.resize_with(len, Default::default);

        IndexGraph { vertices }
    }

    pub fn from_adjacency_list<S>(g: &[S]) -> Self
        where S: AsRef<[usize]>
    {
        IndexGraph::from_graph(g, |mut builder, edges| for &edge in edges.as_ref() {
            builder.add_out_edge(edge)
        })
    }

    pub fn from_graph<T, F>(g: &[T], mut f: F) -> Self
        where F: FnMut(IndexGraphBuilder<'_>, &T)
    {
        let mut graph = Self::with_vertices(g.len());

        for (idx, element) in g.iter().enumerate() {
            f(IndexGraphBuilder { graph: &mut graph, index: idx }, element)
        }

        graph
    }

    pub fn iter(&self) -> SliceIter<'_, Vertex> {
        self.vertices.iter()
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.vertices[from].out_degree += 1;
        self.vertices[to].in_degree += 1;
        self.vertices[from].out_edges.push(to);
        self.vertices[to].in_edges.push(from);
    }

    pub fn transpose(&mut self) {
        for vertex in &mut self.vertices {
            mem::swap(&mut vertex.in_degree, &mut vertex.out_degree);
            mem::swap(&mut vertex.in_edges, &mut vertex.out_edges);
        }
    }

    fn try_toposort_internal(mut self) -> Result<Vec<usize>, IndexGraph> {
        let mut queue = Queue::new();
        let mut sorted = Vec::new();

        // Kahn's algorithm for toposort

        // enqueue vertices with in-degree zero
        for (idx, vertex) in self.vertices.iter_mut().enumerate() {
            // out_degree is unused in this algorithm
            // set out_degree to zero to be used as a 'visited' flag by
            // Kosaraju's algorithm later
            vertex.out_degree = 0;

            if vertex.in_degree == 0 {
                queue.push_back(idx);
            }
        }

        // add vertices from queue to sorted list
        // decrement in-degree of neighboring edges
        // add to queue if in-degree zero
        while let Some(idx) = queue.pop_front() {
            sorted.push(idx);

            for edge_idx in 0..self.vertices[idx].out_edges.len() {
                let next_idx = self.vertices[idx].out_edges[edge_idx];

                self.vertices[next_idx].in_degree -= 1;
                if self.vertices[next_idx].in_degree == 0 {
                    queue.push_back(next_idx);
                }
            }
        }

        // if every vertex appears in sorted list, sort is successful
        if sorted.len() == self.vertices.len() {
            Ok(sorted)
        } else {
            Err(self)
        }
    }

    pub fn try_toposort(self) -> Result<Vec<usize>, IndexGraph> {
        self.try_toposort_internal()
            .map_err(|mut graph| {
                for vertex in graph.vertices.iter_mut() {
                    vertex.in_degree = vertex.in_edges.len();
                    vertex.out_degree = vertex.out_edges.len();
                }

                graph
            })
    }

    pub fn toposort(self) -> Option<Vec<usize>> {
        self.try_toposort_internal().ok()
    }

    fn scc_internal(mut self) -> Vec<Vec<usize>> {
        // assumes out_degree is zero everywhere, to be used as a 'visited' flag

        // empty graphs are always cycle-free
        if self.vertices.is_empty() {
            return Vec::new()
        }

        // Kosaraju's algorithm for strongly connected components

        // start depth-first search with first vertex
        let mut queue = Queue::new();
        let mut dfs_stack = vec![(0, 0)];
        self.vertices[0].out_degree = 1;

        // add vertices to queue in post-order
        while let Some((idx, edge_idx)) = dfs_stack.pop() {
            if edge_idx < self.vertices[idx].out_edges.len() {
                dfs_stack.push((idx, edge_idx + 1));

                let next_idx = self.vertices[idx].out_edges[edge_idx];
                if self.vertices[next_idx].out_degree == 0 {
                    self.vertices[next_idx].out_degree = 1;
                    dfs_stack.push((next_idx, 0));
                }
            } else {
                queue.push_back(idx);
            }
        }

        // collect cycles by depth-first search in opposite edge direction
        // from each vertex in queue
        let mut cycles = Vec::new();
        while let Some(root_idx) = queue.pop_back() {
            if self.vertices[root_idx].out_degree == 2 {
                continue
            }

            let mut cur_cycle = Vec::new();

            dfs_stack.push((root_idx, 0));

            while let Some((idx, edge_idx)) = dfs_stack.pop() {
                if edge_idx < self.vertices[idx].in_edges.len() {
                    dfs_stack.push((idx, edge_idx + 1));

                    let next_idx = self.vertices[idx].in_edges[edge_idx];
                    if self.vertices[next_idx].out_degree == 1 {
                        self.vertices[next_idx].out_degree = 2;
                        dfs_stack.push((self.vertices[idx].in_edges[edge_idx], 0));
                        cur_cycle.push(next_idx);
                    }
                }
            }

            if self.vertices[root_idx].out_degree == 2 {
                cycles.push(cur_cycle);
            } else {
                self.vertices[root_idx].out_degree = 2;
            }
        }

        // return collected cycles
        cycles
    }

    pub fn scc(mut self) -> Vec<Vec<usize>> {
        for vertex in self.vertices.iter_mut() {
            vertex.out_degree = 0;
        }

        self.scc_internal()
    }

    pub fn toposort_or_scc(self) -> Result<Vec<usize>, Vec<Vec<usize>>> {
        match self.try_toposort_internal() {
            Ok(sorted) => Ok(sorted),
            Err(graph) => Err(graph.scc_internal())
        }
    }
}

impl Index<usize> for IndexGraph {
    type Output = Vertex;

    fn index(&self, index: usize) -> &Vertex {
        &self.vertices[index]
    }
}

impl<'g> IntoIterator for &'g IndexGraph {
    type Item = &'g Vertex;
    type IntoIter = SliceIter<'g, Vertex>;

    fn into_iter(self) -> Self::IntoIter {
        self.vertices.iter()
    }
}

impl IntoIterator for IndexGraph {
    type Item = Vertex;
    type IntoIter = VecIntoIter<Vertex>;

    fn into_iter(self) -> Self::IntoIter {
        self.vertices.into_iter()
    }
}