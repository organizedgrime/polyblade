<div align="center" style="margin-top: 24px;">
    <div>
        <h1>Polyblade</h1>
        <span>Cross-platform application for animating Conway Polyhedron Operations</span>
    </div>
    <br/>
    <a href="https://github.com/organizedgrime/polyblade/releases/latest"> 
        <img src="https://img.shields.io/github/v/release/organizedgrime/polyblade?style=for-the-badge&color=yellow"/>
    </a>
    <a href="https://github.com/organizedgrime/polyblade/actions/workflows/ci.yml"> 
        <img src="https://img.shields.io/github/actions/workflow/status/organizedgrime/polyblade/ci.yml?style=for-the-badge&label=CI%20Status"/>
    </a>
    <a href="LICENSE"> 
        <img src="https://img.shields.io/badge/license-APGL3-blue.svg?style=for-the-badge"/>
    </a>
    <br/>
    <a href="https://github.com/iced-rs/iced">
        <img alt="Made with Iced" src="assets/made-with-iced.svg" height="36px" />
    </a>
</div>

![demo](./assets/demo.gif)

Polyblade is software for visualizing Polyhedra. 

In particular, emphasis has been placed on making smooth animations for the transitions represented by [Conway Polyhedron Notation](https://en.wikipedia.org/wiki/Conway_polyhedron_notation). 
Polyblade runs on [WGPU](https://github.com/gfx-rs/wgpu) and [Iced](https://github.com/iced-rs/iced).
Using the PST distance algorithm for efficient all pairs shortest paths in the unweighted undirected graphs represented by polyhedra, none of the vertex position data is deterministic. Instead, this distance matrix is used to create spring forces between every node pair $v_i, v_j$ where $v_n \in G$. Proportional to their distance in the graph structure $G$, the springs inflate the polyhedron to proportional size, allowing us to visualize these structures even when they break convexity. 


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
- [x] Create WASM deployment and add to website as git submodule
- [x] Setup some basic CI integrations
