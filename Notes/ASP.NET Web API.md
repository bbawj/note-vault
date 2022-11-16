---
title:"ASP.NET Web API"
---
# ASP.NET Web API
## Routing
Routing uses `APIController`
Add an attribute to a controller class to tell the compiler that this is an `APIController`

```csharp
[APIController]
public class TicketsController: ControllerBase{}
```

#### Route pattern using _attribute binding_
- `IActionResult` is a generic interface to encapsulate all the return types such as XML, JSON.
- Use the route method attribute to set the endpoint
```csharp
[HTTPGet]
[Route("api/tickets")]
public IActionResult GetTickets(){
	return Ok("Reading tickets");
}
//with interpolation
[HTTPGet]
[Route("api/tickets/{id}")]
public IActionResult GetTicket(int id){
	return Ok($"Reading tickets #{id}");
}
```

Can also define the route based on the controller name at the class level
```csharp
[APIController]
[Route("api/[controller]")]
public class TicketsController: ControllerBase{
	[HTTPGet]
	public IActionResult GetTickets(){
		return Ok("Reading tickets");
	}
	
	[HTTPGet("{id}")]
	public IActionResult GetTicket(int id){
		return Ok($"Reading tickets #{id}");
	}
}
```

#### Route pattern using _model binding_
[Primitive type binding](https://docs.microsoft.com/en-us/aspnet/core/mvc/models/model-binding?view=aspnetcore-6.0#sources)
- `FromRoute` 
- `FromQuery`: specifies that this attribute must come from the query string
```csharp
[HTTPGet]
[Route("/api/projects/{pid}/tickets")] //slash at the start indicates from root rather than the controller route defined in the class-level
public IActionResult GetTicketFromProject(int pid, [FromQuery] int tid){
	if (tid == 0){
		return Ok($"Reading all tickets belonging to project #{pid}");
	}
	else {
		return Ok($"Reading project {pid}, tickets #{id}");
	}
}
```

Using a complex type:
```csharp
public class Ticket{
	[FromQuery(Name="tid")]
	public int TicketId {get; set;}

	[FromRoute(Name="pid")]
	public int ProjectId {get; set;}
}

[HTTPGet("{id}")]
[Route("/api/projects/{pid}/tickets")]
public IActionResult GetTicketFromProject(Ticket ticket){
	if (ticket.TicketId == 0){
		return Ok($"Reading all tickets belonging to project #{ticket.ProjectId}");
	}
	else {
		return Ok($"Reading project {ticket.ProjectId}, tickets #{ticket.TickedId}");
	}
}
```

#### Post Routes
```csharp
[HttpPost]
public IActionResult Post([FromBody] Ticket ticket){
	return Ok(ticket); //automatically serializes the body into JSON
}
```

#### Validation
[Data Annotations](https://docs.microsoft.com/en-us/aspnet/core/mvc/models/model-binding?view=aspnetcore-6.0#sources)
Place validation on the mode attributes
```csharp
public class Ticket{
	[Required]
	public int TicketId {get; set;}

	[Required]
	public int ProjectId {get; set;}
}
```

Custom validation attributes
```csharp
public class Ticket_EnsureDueDateForTicketOwner: ValidationAttribute{
	protected override ValidationResult IsValid(Object value, ValidationContext 
	validationContext){
		var ticket = validationContext.ObjectInstance as Ticket;
		if (ticket != null && !string.IsNullOrWhiteSpace(ticket.Owner)){
			if (!ticket.DueDate.HasValue){
				return new ValidationResult("Due date is required when ticket has owner");
			}
		}
		return ValidationResult.Success;
	}
}

/**some code **/

public string Owner {get; set;}

[Ticket_EnsureDueDateForTicketOwner]
public DateTime? DueDate {get; set;}
```

## Filters
How the filter pipeline works:
![](https://docs.microsoft.com/en-us/aspnet/core/mvc/controllers/filters/_static/filter-pipeline-2.png?view=aspnetcore-6.0)

#### Action Filters
[Place validation on endpoint routes.](https://docs.microsoft.com/en-us/aspnet/core/mvc/controllers/filters?view=aspnetcore-6.0#implementation)

```csharp
public class ValidateModelAttribute : ActionFilterAttribute
{
    public override void OnActionExecuting(ActionExecutingContext context)
    {
	    //custom validation on the endpoint
        if (!context.ModelState.IsValid)
        {
	        context.ModelState.AddModelError("SomeKey", "Key is missing");
	        //short circuit the request
            context.Result = new BadRequestObjectResult(context.ModelState);
        }
    }
}

[HttpPost]
[ValidateModelAttribute]
public IActionResult Post([FromBody] Ticket ticket){
	return Ok(ticket); //automatically serializes the body into JSON
}
```

#### [Resource Filters](https://docs.microsoft.com/en-us/aspnet/core/mvc/controllers/filters?view=aspnetcore-6.0#resource-filters)
Useful to short-circuit the rest of the pipeline such as during versioning and caching

```csharp
public class Version1DiscontinueResourceFilter : Attribute, IResourceFilter{
	public void OnResourceExecuting(ResourceExecutingContext context){
		if (path contains v1){
			contex.Result = new BadRequestObjectResut(
								new {
									Versioning = new[] {"This API Version is discontinued"}
								}
							);
		}
	}
}
```

