---
title: "Angular"
date: 2022-11-08
lastmod: 2022-11-21
---
# Angular
A frontend development platform built on [TypeScript](Notes/TypeScript.md).

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
```
ng generate service <name>

ng g s <name>
```

### Dependency Injection
Angular uses [Dependency Injection](Notes/Dependency%20Injection.md) to increase modularity.

Use [](Notes/Dependency%20Injection.md#Constructor%20injection%20%7CConstructor%20Injection) to utilize a service:
```typescript
export class ProductDetailsComponent implements OnInit {

  constructor(
    private route: ActivatedRoute,
    private cartService: CartService
  ) { }
}
```

### Hitting APIs
Configure [HTTPModule](https://angular.io/start/start-data#configure-appmodule-to-use-httpclient)

Data is passed from services to components via [Observables](https://angular.io/guide/observables)
#### [Get](https://angular.io/guide/http#requesting-data-from-a-server)
```typescript
class SomeService{
	constructor(private http: HttpClient){}
	get(): Observable<Task[]>{
		return this.http.get<Task[]>(this.apiUrl)
	}
}
```
#### [Post](https://angular.io/guide/http#making-a-post-request)
#### [Delete](https://angular.io/guide/http#making-a-delete-request)
#### [Put](https://angular.io/guide/http#making-a-put-request)
#### [Error handling](https://angular.io/guide/http#handling-request-errors)

## RxJS Observables
Makes use of the [Observer Pattern](Notes/Observer%20Pattern.md).

<iframe width="560" height="315" src="https://www.youtube.com/embed/T9wOu11uU6U" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

We use `Observables` when there is some stream of data that is changing and we have multiple subscribers that want to change when there is some new data.
![500](https://i.imgur.com/aoiTL89.png)

RxJS provides multiple functions to modify how often we want to call next(), in what ways to format the data etc. 
