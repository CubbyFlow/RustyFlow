# RustyFlow

<img src="./medias/logo.png" width=256 height=256 />

[![License](https://img.shields.io/badge/Licence-MIT-blue.svg)](https://github.com/utilForever/RustyFlow/blob/master/LICENSE) [![Build Status](https://travis-ci.com/utilForever/RustyFlow.svg?branch=master)](https://travis-ci.com/utilForever/RustyFlow) [![Build status](https://ci.appveyor.com/api/projects/status/fse0ft9743ux10an/branch/master?svg=true)](https://ci.appveyor.com/project/utilForever/rustyflow/branch/master) [![Build Status](https://dev.azure.com/utilforever/RustyFlow/_apis/build/status/utilForever.RustyFlow?branchName=master)](https://dev.azure.com/utilforever/RustyFlow/_build/latest?definitionId=15&branchName=master) ![Rust](https://github.com/utilForever/RustyFlow/workflows/Rust/badge.svg?branch=master)

[![codecov](https://codecov.io/gh/utilForever/RustyFlow/branch/master/graph/badge.svg)](https://codecov.io/gh/utilForever/RustyFlow)

RustyFlow is re-implementation of [CubbyFlow](https://github.com/utilForever/RustyFlow) in Rust. It is voxel-based fluid simulation engine for computer games based on [Jet framework](https://github.com/doyubkim/fluid-engine-dev) that was created by [Doyub Kim](https://twitter.com/doyub).

## To-do Features

- Basic math and geometry operations and data structures
- Spatial query accelerators
- SPH and PCISPH fluid simulators
- Stable fluids-based smoke simulator
- Level set-based liquid simulator
- PIC, FLIP, and APIC fluid simulators
- Upwind, ENO, and FMM level set solvers
- Jacobi, Gauss-Seidel, SOR, MG, CG, ICCG, and MGPCG linear system solvers
- Spherical, SPH, Zhu & Bridson, and Anisotropic kernel for points-to-surface converter
- Converters between signed distance function and triangular mesh

Every simulator has both 2-D and 3-D implementations.

## Documentation

TBA

## Examples

TBA

## How To Contribute

Contributions are always welcome, either reporting issues/bugs or forking the repository and then issuing pull requests when you have completed some additional coding that you feel will be beneficial to the main project. If you are interested in contributing in a more dedicated capacity, then please contact me.

## Contact

You can contact me via e-mail (utilForever at gmail.com). I am always happy to answer questions or help with any issues you might have, and please be sure to share any additional work or your creations with me, I love seeing what other people are making.

## License

<img align="right" src="http://opensource.org/trademarks/opensource/OSI-Approved-License-100x137.png">

The class is licensed under the [MIT License](http://opensource.org/licenses/MIT):

Copyright &copy; 2020 [Chris Ohk](http://www.github.com/utilForever)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
