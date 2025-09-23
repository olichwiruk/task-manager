# Task Manager

A sample task management application built with Rust, showcasing server-side rendering and a clean architecture design.

## Live Demo

Check out the [live demo](https://task-manager.olichwir.uk/) to see the application in action.

## Tech Stack

- **Backend**: Rust with Axum web framework
- **Database**: PostgreSQL with SQLx for type-safe async queries
- **Frontend**:
  - HTMX for dynamic client-side interactions without JavaScript frameworks
  - Tailwind CSS with DaisyUI for responsive UI components
- **Templating**: Askama for server-side rendering
- **Authentication**: Basic authentication with bcrypt password hashing
- **Runtime**: Tokio async runtime
- **Configuration**: Environment variables via dotenvy

## Architecture

The application follows a clean, layered architecture with clear separation of concerns:

### Domain Layer
Contains the core business logic and entities:
- `domain/task.rs`: Task entity definition and associated logic
- `domain/user.rs`: User entity definition and associated logic

### Application Layer
Implements use cases that coordinate the flow of data

### Presentation Layer
Manages the interface between the system and users:
- `views/tasks.rs`: Task view logic
- `views/users.rs`: User view logic
- `templates/*.html`: HTML templates rendered with Askama

### Infrastructure Layer
Provides technical capabilities that support higher layers:
- `app/routes.rs`: API route definitions
- `app/state.rs`: Application state management
- `infrastructure/repositories/sqlx_repository.rs`: Database access through SQLx
- `handlers/task_handler.rs`: Task-related request handling
- `handlers/user_handler.rs`: User-related request handling

## Implemented Features

- **User Authentication**: Registration functionality with password hashing
- **Task Management**: Create, read, update, and delete tasks
- **Server-Side Rendering**: HTML generated on the server using Askama templates
- **Database Integration**: PostgreSQL with SQLx for type-safe queries
- **Database Migrations**: SQL-based schema evolution
- **Error Handling**: Structured error handling throughout the application
- **Responsive UI**: Basic responsive design

## Future Enhancements

- Additional Features:
   - Role-based Task Management:
     - Admin role with privileges to assign users to tasks and edit any task
     - User role limited to updating status of tasks assigned to them
   - Multi-language support
   - Search functionality
   - Notifications system
- Add comprehensive input validation
- Enhance error handling with detailed diagnostics and recovery strategies
- Implement OAuth2/OIDC authentication flow
- Integrate Kafka for event-driven communication
- Observability:
   - Add structured logging
   - Implement metrics collection
   - Set up distributed tracing with OpenTelemetry
- DevOps Integration:
   - Implement CI/CD pipeline
   - Set up monitoring and alerting

## Getting Started

### Prerequisites
- Rust
- PostgreSQL database

### Setup

1. Clone the repository
2. Create a `.env` file with the following variables:
   ```
   DATABASE_URL=postgres://username:password@localhost/task_manager
   ```
3. Run database migrations:
   ```
   sqlx database create
   sqlx migrate run
   ```
4. Build and run the application:
   ```
   cargo run
   ```
5. Access the application at `http://localhost:3000`
