# Important API

This important API is a demo application that shows a fully-functional [Atmo](https://github.com/suborbital/atmo) app. It showcases the Runnable API, and the ease of creating a WebAssembly server application using declarative development.

To run, you'll need to install [subo](https://github.com/suborbital/subo), the Suborbital CLI.

Running this application only takes two commands:

```
> subo build . --bundle
> subo dev
```

The application will run on port 8080. Request `GET localhost:8080/stars/suborbital/atmo` to try it out.

The entire app is controlled by the [Directive](./Directive.yaml), which declaratively describes the application's business logic.

The `ghstars` Runnable uses the Runnable API to fetch data from the GitHub API and returns it to the caller.

Subo builds a **Runnable Bundle** containing the Directive and all of the Runnables (compiled to WebAssembly), and then Atmo uses the Bundle to run your application.

**To learn more, visit the [Atmo guide](https://atmo.suborbital.dev)**