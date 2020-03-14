fn main() {
    
}

struct Edge {
    src: usize,
    dest: usize,
  }
  
  struct Graph {
      V: usize, // Vertices
      E: usize, // Edges
      edge: Vec<Edge> // Array of edges 
  }
  
  #[derive(Clone, Copy)]
  struct Subset {
      parent: usize,
      rank:  usize
  }
  
  // A very basic implementation of Karger's randomized 
  // algorithm for finding the minimum cut. Please note 
  // that Karger's algorithm is a Monte Carlo Randomized algo 
  // and the cut returned by the algorithm may not be 
  // minimum always 
  fn kargerMinCut(graph: Graph) -> usize
  { 
      // Get data of given graph 
      let V = graph.V;
      let E = graph.E; 
      let edge = graph.edge; 
    
      // Allocate memory for creating V subsets. 
      let mut subsets: Vec<Subset> = Vec::with_capacity(V); 
    
      // Create V subsets with single elements 
      for v in 0..V { 
          subsets[v].parent = v; 
          subsets[v].rank = 0; 
      } 
    
      // Initially there are V vertices in 
      // contracted graph 
      let mut vertices = V; 
    
      // Keep contracting vertices until there are 
      // 2 vertices. 
      while vertices > 2 
      { 
         // Pick a random edge 
         let i = rand::random::<usize>() % E; 
    
         // Find vertices (or sets) of two corners 
         // of current edge 
         let subset1 = find(subsets.clone(), edge[i].src); 
         let subset2 = find(subsets.clone(), edge[i].dest); 
    
         // If two corners belong to same subset, 
         // then no point considering this edge 
         if subset1 == subset2 { continue; }
    
         // Else contract the edge (or combine the 
         // corners of edge into one vertex) 
         else { 
            println!("Contracting edge {}-{}\n", 
                   edge[i].src, edge[i].dest); 
            vertices = vertices - 1; 
            union(subsets.clone(), subset1, subset2); 
         } 
      } 
    
      // Now we have two vertices (or subsets) left in 
      // the contracted graph, so count the edges between 
      // two components and return the count. 
      let mut cutedges = 0; 
      for i in 0..E { 
          let subset1 = find(subsets.clone(), edge[i].src); 
          let subset2 = find(subsets.clone(), edge[i].dest); 
          if subset1 != subset2  { cutedges = cutedges + 1; } 
      } 
    
      return cutedges; 
  } 
  
  
  fn find(mut subsets: Vec<Subset>, i: usize) -> usize {
      if subsets[i].parent != i { 
          let parent = &subsets[i].parent;
          subsets[i].parent = find(subsets.clone(), *parent);
       }
  
      return subsets[i].parent;
  }
  
  fn union(mut subsets: Vec<Subset>, x: usize, y: usize) 
  { 
      let xroot: usize = find(subsets.clone(), x); 
      let yroot: usize = find(subsets.clone(), y); 
    
      // Attach smaller rank tree under root of high 
      // rank tree (Union by Rank) 
      if subsets[xroot].rank < subsets[yroot].rank {
          subsets[xroot].parent = yroot;
      }
  
      else if subsets[xroot].rank > subsets[yroot].rank { 
          subsets[yroot].parent = xroot; 
      }
    
      // If ranks are same, then make one as root and 
      // increment its rank by one 
      else
      { 
          subsets[yroot].parent = xroot; 
          subsets[xroot].rank = subsets[xroot].rank + 1; 
      } 
  } 
