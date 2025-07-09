use axum::{extract::Path, Json};
use crate::models::{Exercise, Rest, Workout, WorkoutItem};

/// Handler to get a workout for a specific date.
/// For now, it returns a hardcoded mock workout.
pub async fn get_workout_by_date(Path(date): Path<String>) -> Json<Workout> {
    println!("->> GET /api/v1/workouts/{}", date);

    // Mock data for demonstration purposes
    let mock_workout = Workout {
        date: date,
        items: vec![
            WorkoutItem::Exercise(Exercise {
                name: "Push Ups".to_string(),
                duration: 60,
            }),
            WorkoutItem::Rest(Rest { duration: 30 }),
            WorkoutItem::Exercise(Exercise {
                name: "Squats".to_string(),
                duration: 90,
            }),
        ],
    };

    Json(mock_workout)
}
