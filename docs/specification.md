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
  * [Areas of responsibility](#areas-of-responsibility)
  
## Useful links
* [Project page](https://github.com/orgs/IndaPlus22/projects/1/views/1)
* [Issues](https://github.com/IndaPlus22/sakao-jblomlof-tarnvik-daniels5-project/issues)

## Branches
The branch structure for this project is the following. Ordered from top to bottom, depending on importance.

main - The main branch shall always be in a functional state. It's protected from pushes, only merging can be done.

dev - The branch that merges all sub-branches. Only one branch on this layer. When in a good state this branch shall be merged to main.

issue - For each issue a new branch is created to deal with that issue. It's then merged to dev.

## Project standards
When working on issues include the issue number. Pull-requests should be always be created on a new branch with the branch named after the issue. If the issue is (#9 "Do this"), then the branch shall be named "issue/9-do-this". If you want to create a new branch for an issue that already exists, add a -2 to the end. "issue/9-do-this-2", and so on.  
Issues should have a short but descriptive title with a longer description describing the issue. Title should be in future tense.

Create a new branch with `git switch -c <branch-name>`. The `-c` flag creates a new branch.

Incase you want to make a sub-branch of an issue-branch name that branch "issue/feature/9-do-this".

Merge's are supposed to be done by the assigne.

#### Milestones
There will be milestones of both weekly-progress and milestones over system-requirements.

## Project

### Idea
The project aims to create a 2-dimensional physics engine, where you are able to spawn in objects. The objects can be simple shapes, or complex shapes consisting of simple shapes combined with joints. The end goal is too be able to place more advanced objects such as springs and thrusters.

### Feasibillity
Since we are 4 devolopers working on the project and creating a simple 2d program we deem it feasibile to accomplish this.

### Workflow


### Areas of responsibility
This will be what each person first and foremost will do. Ofcourse no one is limited to each area and we will help where help is needed.

| Area | Person |
|------|--------|
| Front-end | Toshihide <sakao@kth.se>, Tilde <tarnvik@kth.se> |
| Back-end | Daniel <daniels5@kth.se>, Jonathan <jblomlof@kth.se> |
