use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Modélisation simple d’une règle de génération de tâches.
///
/// Pour commencer, les règles pourront être chargées depuis un fichier JSON/YAML
/// et évaluées par le moteur de règles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: Uuid,
    pub key: String,
    pub name: String,
    pub description: String,
    /// Expression ou mini DSL décrivant la condition (sera interprétée côté moteur).
    pub condition: String,
    /// Modèle de titre de tâche, ex: "Active le 2FA sur {{ app }}".
    pub task_title_template: String,
    /// Modèle de description de tâche.
    pub task_description_template: String,
    /// Points attribués si la tâche est complétée.
    pub points: i32,
    /// Catégorie fonctionnelle de la tâche générée (MFA, Device, etc.).
    pub task_type: String,
    /// Règle active ou désactivée.
    pub enabled: bool,
}


