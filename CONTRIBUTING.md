# Contributing to the future

Hello there! Thank you for considering to contribute to the project.
This file/page is destined to evolve with the time, I will try to keep it updated with all the style/standard choices I do for the code of the repository and their rationals.

I'm using this project to learn more about both React and Rust. Don't hesitate to check their amazing documentation [here](https://reactjs.org/docs/getting-started.html) and [here](https://doc.rust-lang.org/book/title-page.html) if you need so.

## Project board, issues and branches

The project is using the [Project feature from github](https://github.com/features/project-management/). [On this board](https://github.com/Virtual-felix/phoenix/projects/1) you can find the backlog containing most of the future work to do (big picture), all the defined task to perform (detailed) and the work in progress.

The lifetime of a ticket is basically:

1. Idea/Note in the backlog
2. Create one or multiple detailed issue to be done to make it real
3. Move them in the Todo column when it is planned to be done
4. Create a branch with the name of the issue
5. Move the ticket in In progress
6. Once it is ready, open a pull request and link it to the issue
7. The pull request is reviewed
8. Once we merge the PR, the issue/ticket is moved to the Done column

Every issues (and so branches) are prefixed with PH-XX (XX being a number). It is done manualy for now but I will see if it can be automatized.

## Code of Conduct

Embrace the differences, be kind and loveful <3.
Don't be bad.

# React

## Folder tree

First, I kept the generated base from [CRA](https://reactjs.org/docs/create-a-new-react-app.html). I won't explain, it is quite intuitive and you can learn more on the documentation.

All the app code is in the `/src` folder.
The assets are in their own folder `/src/asset` to avoid mixing javascript and non-javascript items. In a context where there would be other people than developpers working on the project, it would also make them more reachable for them.
You will find in `/src/util` all the helper functions that are using among all the components. Lets keep the code DRY (Don’t Repeat Yourself).

Then, the `/src/component` folder. This is the most important part. Every component in the project will have the own directory in this folder. And in each of those directories, you will have a `index.js` file exporting the public parts of those components and as default one the top most main component (and only do this), then the code will be in the other files of the directory. This way, you can import the component using the folder name `src/component/component-name` which make it much better for maintainability (you can refactor/split your code as many time as you want without any changes on the imports).
Furthermore, all the component will have this file structure:

- The state-less component in `component-name-view.js`
- The container in `component-name-container.js`
- The global state component in `component-name-redux.js`
- Local specific helpers in `component-name-util.js`
  It makes is easier for the components to evolve from state-less to more complex ones without moving code from a file to another one and lose track of the code in the version control system.

Most of those rules can be nested (capped at depth of 1). It means that if you have a component which is strictly coupled to another one as a child, you can nest it in the parent component folder and follow all the abose rules.

Kudo to [Charles Stover](https://github.com/CharlesStover) and his [really good react guide for a nice folder structure](https://medium.com/@Charles_Stover/optimal-file-structure-for-react-applications-f3e35ad0a145) which I used for this project.

## Testing

Coming soon
