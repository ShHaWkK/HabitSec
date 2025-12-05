/// Calcule un score de conformité humaine (0–100) à partir :
/// - des points complétés
/// - des points disponibles
/// - d’une pénalité moyenne liée au retard.
///
/// Ce module est pur (pas d’accès DB) et facilement testable.
pub fn compute_user_score(
    completed_points: i32,
    available_points: i32,
    average_delay_days: f32,
) -> i32 {
    if available_points <= 0 {
        return 0;
    }

    let base = (completed_points as f32 / available_points as f32) * 100.0;

    // Pénalité douce : 2 points par jour de retard moyen, bornée.
    let penalty = (average_delay_days * 2.0).clamp(0.0, 40.0);

    let score = (base - penalty).clamp(0.0, 100.0);
    score.round() as i32
}

#[cfg(test)]
mod tests {
    use super::compute_user_score;

    #[test]
    fn score_is_zero_when_no_available_points() {
        assert_eq!(compute_user_score(0, 0, 0.0), 0);
    }

    #[test]
    fn full_completion_without_delay_gives_100() {
        assert_eq!(compute_user_score(100, 100, 0.0), 100);
    }

    #[test]
    fn partial_completion_scales_linearly() {
        assert_eq!(compute_user_score(50, 100, 0.0), 50);
    }

    #[test]
    fn delay_applies_penalty_but_is_bounded() {
        // 100 points complétés, mais 10 jours de retard -> 20 points de pénalité.
        assert_eq!(compute_user_score(100, 100, 10.0), 80);

        // Pénalité max 40 points même pour un retard énorme.
        assert_eq!(compute_user_score(100, 100, 100.0), 60);
    }
}


