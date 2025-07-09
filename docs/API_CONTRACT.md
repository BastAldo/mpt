# API Contract v1

This document defines the application programming interface (API) for "Mio Trainer Personale". The frontend and backend MUST strictly adhere to these specifications.

**Base URL:** `(to be defined)`

---

## Common Data Types

### Workout
```json
{
  "date": "YYYY-MM-DD",
  "items": [
    // Array of Exercise or Rest objects
  ]
}
```

### Exercise
```json
{
  "type": "exercise",
  "name": "Exercise Name",
  "duration": 60 // in seconds
}
```

### Rest
```json
{
  "type": "rest",
  "duration": 30 // in seconds
}
```

---

## Endpoints

*(No endpoints defined yet)*