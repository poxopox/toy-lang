A toy language I started writing in 2022. It's very basic and only supports reading in integers and floats.

The ultimate goal is to implement a new "full stack" language that allows easy implementation of web applications
that mixes frontend and backend technologies and allows shared state between them.

For Example:

```html5
<script type="text/javascript" tech="nodejs" name="backend">

  export const someData = mongoService.getData();

</script>

<script type="application/typescript" tech="angular" name="frontend">
import { Component } from '@angular/core';

@Component({
  selector: 'app-root',
  template: '<h1>{{ someData }}</h1>',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  someData = backend.someData;
}
</script>

```

When built, this will create a web application that displays the data retrieved from the backend.
The backend state will be shared with the frontend through a REST API with generated integrations.
