<div align="center" style="margin-top: 24px;">
    <div>
        <h1>Polyblade</h1>
        <span>Cross-platform application for animating Conway Polyhedron Operations</span>
    </div>
    <br/>
    <a href="https://vera.lgbt/software/polyblade_live/index.html">
        <img src="https://img.shields.io/website?url=https%3A//vera.lgbt/software/polyblade_live/index.html&label=demo&logo=webgl&logoSize=auto&style=for-the-badge"/>
    </a>
    <a href="https://github.com/organizedgrime/polyblade/actions/workflows/ci.yml"> 
        <img src="https://img.shields.io/github/actions/workflow/status/organizedgrime/polyblade/ci.yml?style=for-the-badge&label=CI%20Status"/>
    </a>
    <a href="LICENSE"> 
        <img src="https://img.shields.io/badge/license-APGL3-blue.svg?style=for-the-badge"/>
    </a>
    <br/>
</div>
<br/>
<div>
    <p>
        <img src="./assets/demo.gif" align="right" alt="animated" width="20%" />
        Polyblade makes it easy to visualize and interact with Polyhedra. I believe that the relationships between both primitive and complex polytopes can be more intuitively understood when experienced visually, and this software aims to demonstrate that.
        In particular, emphasis has been placed on making smooth animations for the transitions represented by [Conway Polyhedron Notation](https://en.wikipedia.org/wiki/Conway_polyhedron_notation). 
        Polyblade runs on [WGPU](https://github.com/gfx-rs/wgpu) and [Iced](https://github.com/iced-rs/iced).
        Using the PST distance algorithm for efficient all pairs shortest paths in the unweighted undirected graphs represented by polyhedra, none of the vertex position data is deterministic. Instead, this distance matrix is used to create spring forces between every node pair $v_i, v_j$ where $v_n \in G$. Proportional to their distance in the graph structure $G$, the springs inflate the polyhedron to proportional size, allowing us to visualize these strucutres even when they break convexity. 
        To run this software, simply clone the repository and use `cargo run --release`.
        The `webgl` demo is available, but is notably less performant than native code. 
    </p>
</div>

#### Conway Roadmap
- [x] Ambo
- [x] Kis
- [x] Truncate
- [ ] Ortho
- [x] Bevel
- [x] Expand
- [ ] Snub
- [x] Join
- [ ] Zip
- [ ] Gyro
- [ ] Meta
- [ ] Needle

#### Other goals
- [x] Replace all hardcoded presets with prisms, antiprisms, and pyramids that have undergone modification.
- [ ] Implement Vertex Coloring and Edge Coloring
- [ ] Fix Fibonnaci lattice distribution for new shapes
- [ ] Tesselations / tilings using Wythoff
- [ ] "Undo" button
- [ ] Save and load animations and cycles of `Transaction`s
- [x] Schlegel diagrams
- [x] Color pickers
- [x] Pokedex entries for polyhedra, point users to wikipedia or polytope wiki when they stumble onto a known entry
  - [ ] Expand pokedex to include more shapes and improve overlap on isomorphic conway strings
  - [ ] Fix pokedex on WASM
- [ ] Create WASM deployment and add to website as git submodule
  - [x] WebGL compat
  - [ ] WebGPU compat
- [x] Setup some basic CI integrations
