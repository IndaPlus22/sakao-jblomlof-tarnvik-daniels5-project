# Specification
This will be the specification for the project [sakao-jblomlof-tarnvik-daniels5-project](https://github.com/IndaPlus22/sakao-jblomlof-tarnvik-daniels5-project/)

## Table of contents
* [Useful links](#useful-links)
* [Branches](#branches)
* [Project standards](#project-standards)
* [Project](#project)
  * [Idea](#idea)
  * [Feasibillity](#feasibillity)
  * [Workflow](#workflow) 
  
## Useful links
* [Project page](https://github.com/orgs/IndaPlus22/projects/1/views/1)
* [Issues](https://github.com/IndaPlus22/sakao-jblomlof-tarnvik-daniels5-project/issues)

## Branches
The branch structure for this project is the following. Ordered from top to bottom, depending on importance.

main - The main branch shall always be in a functional state. It's protected from pushes, only merging can be done.

dev - The branch that merges all sub-branches. Only one branch on this layer. When in a good state this branch shall be merged to main.

issue - For each issue a new branch is created to deal with that issue. It's then merged to dev.

## Project standards
When working on issues include the issue number. Pull-requests should be always be created on a new branch with the branch named after the issue. If the issue is (#9 "Do this"), then the branch shall be named "issue/9-do-this". 
Issues should have a short but descriptive title with a longer description describing the issue.

Create a new branch with `git switch -c <branch-name>`. The `-c` flag creates a new branch.

Merge's are supposed to be done by the assigne.

## Project

### Idea
The project aims to create a 2-dimensional physics engine, where you are able to spawn in objects. The objects can be simple shapes, or complex shapes consisting of simple shapes combined with joints.

### Feasibillity
Since we are 4 devolopers working on the project and creating a simple 2d program we deem it feasibile to accomplish this.

### Workflow
The workflow for creating a 2-dimensional physics engine that allows for the spawning of objects can be broken down into several steps:

* Requirements gathering: Define the scope and requirements of the project. This includes identifying the necessary features such as object spawning, simple and complex shapes, joints, and physics simulation.
* Architecture design: Decide on the architecture of the physics engine, taking into account the requirements gathered in step 1. This includes selecting a programming language and any relevant libraries or frameworks that will be used.
* Object spawning: Implement the ability to spawn objects in the physics engine. This involves creating a data structure that can hold object properties such as position, velocity, mass, and shape.
* Simple shape support: Add support for simple shapes such as rectangles, circles, and triangles. This involves creating the appropriate collision detection algorithms for each shape and updating the object properties based on the laws of physics.
* Complex shape support: Implement the ability to create complex shapes by combining simple shapes with joints. This requires creating a data structure for joints and developing algorithms to update the positions of objects connected by joints.
* Physics simulation: Implement the physics simulation engine. This includes incorporating gravity, friction, and other physical forces into the simulation. It also requires implementing collision detection and resolution algorithms to handle interactions between objects.
* Testing: Test the physics engine by creating and simulating different scenarios. This includes testing object spawning, simple and complex shapes, joints, and physics simulation.
* Refinement: Refine the physics engine based on feedback from testing. This may involve optimizing code, improving algorithms, or adding new features.
* Documentation: Document the physics engine, including how to use it and any limitations or constraints.

Overall, the workflow involves a combination of design, coding, testing, and refinement to create a robust and functional 2-dimensional physics engine.
