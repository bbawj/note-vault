# Angular
A frontend development platform built on [[TypeScript]].

## Creating components
```console
ng generate component <name>

ng g c <name>
```

### Defining metadata
A file in the form of `<name>.component.ts` will be generated.

```typescript
@Component({
  selector:    'app-hero-list',
  templateUrl: './hero-list.component.html',
  providers:  [ HeroService ]
})
export class HeroListComponent implements OnInit {
  /* . . . */
}
```

`selector`

A CSS selector that tells Angular to create and insert an instance of this component wherever it finds the corresponding tag in template HTML. For example, if an application's HTML contains `<app-hero-list></app-hero-list>`, then Angular inserts an instance of the `HeroListComponent` view between those tags.

`templateUrl`

The module-relative address of this component's HTML template. Alternatively, you can provide the HTML template inline, as the value of the `template` property. This template defines the component's _host view_.

`providers`

An array of [providers](https://angular.io/guide/glossary#provider) for services that the component requires. In the example, this tells Angular how to provide the `HeroService` instance that the component's constructor uses to get the list of heroes to display.


## Templating
In a file in the form of `<name>.component.html`.

### Data Binding
![](https://i.imgur.com/emwdOkY.png)

![](https://i.imgur.com/D3fnw4j.png)

#### 2 way binding


### Pipes
We can use pipes to transform values into a specific display format in our view. 

Angular defines various pipes, such as the [date](https://angular.io/api/common/DatePipe) pipe and [currency](https://angular.io/api/common/CurrencyPipe) pipe; for a complete list, see the [Pipes API list](https://angular.io/api?type=pipe). You can also define new pipes.

To specify a value transformation in an HTML template, use the [pipe operator (`|`)](https://angular.io/guide/pipes): 
```
{{interpolated_value | pipe_name}}
```

### Directives
Angular templates are dynamic. When Angular renders them, it transforms the DOM according to the instructions given by directives. A directive is a class with a ``@Directive()`` decorator.

#### Structural directives
They alter layout by adding, removing, and replacing elements in the DOM. [Guide]()

| Directive                                                      | Details                                                                               |
| -------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| [`*ngFor`](https://angular.io/guide/built-in-directives#ngFor) | An iterative; it tells Angular to stamp out one `<li>` per hero in the `heroes` list. |
|[`*ngIf`](https://angular.io/guide/built-in-directives#ngIf)                                                                |A conditional; it includes the `HeroDetail` component only if a selected hero exists.|                                                                                       |

```typescript
<li *ngFor="let hero of heroes"></li>
<app-hero-detail *ngIf="selectedHero"></app-hero-detail>
```

#### Attribute directives
They alter the appearance or behavior of an existing element. In templates they look like regular HTML attributes, hence the name. [Guide](https://angular.io/guide/attribute-directives)

| Directive | Details |
| --------- | ------- |
| [ngModel](https://angular.io/api/forms/NgModel) | `ngModel` modifies the behavior of an existing element (typically `<input>`) by setting its display value property and responding to change events.        |



## Services

### Dependency Injection
Angular uses [[Dependency Injection]] to increase modularity.



