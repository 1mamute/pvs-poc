use std::collections::HashSet;


use rand::Rng;

use crate::entity::{Entity32, Entity64};

pub struct BaselinePVS32 {}

impl BaselinePVS32 {
    pub fn update_entities(current_entities: &mut HashSet<Entity32>, new_entities: &HashSet<Entity32>) {
        // Remover entidades que não estão em new_entities
        current_entities.retain(|e| new_entities.contains(e));
    
        // Adicionar novas entidades de new_entities a actual_entities
        // current_entities.extend(new_entities.iter().cloned());
        // Otimização na adição
        for entity in new_entities {
            if !current_entities.contains(entity) {
                current_entities.insert(entity.clone());
            }
        }
    }

    // Função para gerar entidades aleatórias
    pub fn generate_random_entities(num_entities: usize) -> HashSet<Entity32> {
        let mut rng = rand::thread_rng();
        let mut entities = HashSet::with_capacity(num_entities);

        for _ in 0..num_entities {
            let id: u32 = rng.gen(); // Gera um ID aleatório de u32.
            entities.insert(Entity32 { id });
        }

        entities
    }
}

pub struct BaselinePVS64 {}

impl BaselinePVS64 {
    pub fn update_entities(current_entities: &mut HashSet<Entity64>, new_entities: &HashSet<Entity64>) {
        // Remover entidades que não estão em new_entities
        current_entities.retain(|e| new_entities.contains(e));
    
        // Adicionar novas entidades de new_entities a actual_entities
        // current_entities.extend(new_entities.iter().cloned());
        // Otimização na adição
        for entity in new_entities {
            if !current_entities.contains(entity) {
                current_entities.insert(entity.clone());
            }
        }
    }

    // Função para gerar entidades aleatórias
    pub fn generate_random_entities(num_entities: usize) -> HashSet<Entity64> {
        let mut rng = rand::thread_rng();
        let mut entities = HashSet::with_capacity(num_entities);

        for _ in 0..num_entities {
            let id: u64 = rng.gen(); // Gera um ID aleatório de u64.
            entities.insert(Entity64 { id });
        }

        entities
    }
}