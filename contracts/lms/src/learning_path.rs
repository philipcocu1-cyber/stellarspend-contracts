use soroban_sdk::{contracttype, Address, Env, String, Vec};

use crate::errors::Error;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LearningPath {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub courses: Vec<u64>,
    pub difficulty: Difficulty,
    pub estimated_completion_time: u64,
    pub instructor: Address,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    LearningPath(u64),
    LearningPathCount,
}

pub fn create_learning_path(
    env: &Env,
    instructor: Address,
    title: String,
    description: String,
    courses: Vec<u64>,
    difficulty: Difficulty,
    estimated_completion_time: u64,
) -> Result<u64, Error> {
    instructor.require_auth();

    if title.len() == 0 {
        return Err(Error::InvalidLearningPathTitle);
    }

    if courses.len() == 0 {
        return Err(Error::LearningPathRequiresCourses);
    }

    if estimated_completion_time == 0 {
        return Err(Error::InvalidCompletionTime);
    }

    let id = get_next_path_id(env);

    let path = LearningPath {
        id,
        title,
        description,
        courses,
        difficulty,
        estimated_completion_time,
        instructor,
    };

    env.storage()
        .instance()
        .set(&DataKey::LearningPath(id), &path);

    env.storage()
        .instance()
        .set(&DataKey::LearningPathCount, &(id + 1));

    Ok(id)
}

pub fn get_learning_path(
    env: &Env,
    path_id: u64,
) -> Result<LearningPath, Error> {
    env.storage()
        .instance()
        .get(&DataKey::LearningPath(path_id))
        .ok_or(Error::LearningPathNotFound)
}

pub fn get_next_path_id(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::LearningPathCount)
        .unwrap_or(1)
}