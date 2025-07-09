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

### Get Workout by Date
- **Endpoint:** `GET /api/v1/workouts/{date}`
- **Description:** Retrieves the workout plan for a specific date.
- **URL Parameters:**
  - `{date}`: The date of the workout in `YYYY-MM-DD` format.
- **Success Response (200 OK):**
  - **Body:** A `Workout` object. If no workout exists for that date, returns a `Workout` object with an empty `items` array.
- **Error Response (400 Bad Request):**
  - **Reason:** The date format is invalid.

### Save Workout by Date
- **Endpoint:** `POST /api/v1/workouts/{date}`
- **Description:** Creates or replaces the workout plan for a specific date.
- **URL Parameters:**
  - `{date}`: The date of the workout in `YYYY-MM-DD` format.
- **Request Body:**
  - A `Workout` object. The `date` in the body must match the date in the URL.
- **Success Response (204 No Content):**
  - **Body:** Empty. Indicates the workout was saved successfully.
- **Error Response (400 Bad Request):**
  - **Reason:** The date format is invalid or the request body is malformed.