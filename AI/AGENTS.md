# Summary
Instructions for coding in general, as well as for working on specific project types, and for working in different languages.

## Coding
- Do not add more comments then necessary. In all cases be descriptive with your naming to self document the code that way. Only include comments when something is especially confusing, or unclear
- If the project you are working on has a testing suite, always make sure to run your code against it to make sure that things are working correctly

## Project types

### React Projects
**General React guidelines:**
- Do not use barrel files or reexport anything, unless the project being worked on is a library (in most cases it will not be)

**Tech stack:**
All React projects should utilize the following tech stack:
- vite
- redux toolkit
- scss (sass)
- vitest
- react(latest)
- axios
- eslint
- prettier
- typescript

React projects working with 3D technologies or VR should utilize the following packages to do so:
- three
- @react-three/* (Depending on the project various pmndrs react-three libraries will be used)


**Components:**
All components inside of React projects should be neatly organized and follow the following structure:

```txt
MyComponent(directory)
  - hooks(directory/optional) // contains all hooks that are only specific to this component. (should be rare)
    - useMyHook.ts // Conatins any business logic for the component that cannot be separated from react based code. (should be rare)
  - MyComponent.tsx // Contains the specific UI code for the component (AND ONLY THE UI CODE)
  - MyComponent.module.scss // Contains all of the styling code for the component. Inline styles are to be avoided at all cost.
  - MyComponent.logic.ts // Contains all of the business logic for the component. Should be an exported class named after the component and contain only static methods. All methods in logic.ts files should have 100% test coverage.
  - MyComponent.test.ts(x) // Contains all tests for the component. Should typically just be a ts file and contain the tests for the .logic.ts file. It can contain UX tests for the .tsx file as well, but those are typically slow and should be avoided as much as possible
```

Components should be small and for the most part as simple as possible. 

Projects should also utilize "layout components". These are components that only concern themselves with making sure that UI elements are in the specific locations that they should be. (i.e. three columns). They should not contain any state that doesn't strictly pertain to how elements are displayed. (i.e. state for if a popover menu is showing is okay, but the logic for when it should show should be external)

**tsx files:**
- All imports should be explicit. No relative imports except for the module.scss file
- Tsx components should only import the .module.scss file that pertains to that component. No sharing .module.scss files
- All exports should be named exports, not default exports, except in situations where that is not possible
- In 95% of cases a file tsx file should only contain one export which is the component
- functional components should be used.
- example tsx component: 
```tsx
inteface MyComponentProps {
    myString: string
}
export const MyComponent = ({myString}: MyComponentProps) => {...}
```


### Tauri Projects
When working with tauri most of the heavy lifting business logic should be done in the Rust side of things, and be structured in a way that the business logic is organized into modules that could be moved between projects if need be. All business logic in rust should have 100% test coverage

**Tech Stack:**
All tauri projects should use the following tech stack:
- React for the UI
- typescript
- serde_json for json
- tokio for async await



## Languages

### Typescript
- Respect the existing architecture and coding standards.
- Prefer readable, explicit solutions over clever shortcuts.
- Extend current abstractions before inventing new ones.
- The use of `any` (implicit or explicit) is strictly forbidden. The use of `unknown` is also not allowed. Always use precise and explicit types. 
- Use `async/await`; wrap awaits in try/catch with structured errors.
- Use PascalCase for classes, interfaces, enums, and type aliases; camelCase for everything else.
- Follow the repository's folder and responsibility layout for new code.
- Use pure ES modules; never emit `require`, `module.exports`, or CommonJS helpers.


### Scss
- Avoid z-index like the plague
- Avoid duplicating styles when possible
- Make use of css variables for theming
- NEVER use !important, always make selectors more specific or restructure, do not use the !important tag
