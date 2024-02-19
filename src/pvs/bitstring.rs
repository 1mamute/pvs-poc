use bitvec::array::BitArray;


use bitvec::order::Lsb0;

use bitvec::vec::BitVec;
use rand::Rng;
use bitvec::bitvec;

pub struct BitStringPVS32 {}

impl BitStringPVS32 {
    pub fn update_entities(_current_entities: &mut BitVec, _new_entities: &BitVec) {
        // Assume-se que `current_entities` e `new_entities` têm o mesmo tamanho
        // *current_entities = *new_entities;
    }

    // Função para gerar entidades aleatórias
    pub fn generate_random_entities(num_entities: usize) -> BitVec {
        let mut rng = rand::thread_rng();

        // Cria um `u32` como um campo de bits, inicializado com zeros
        let mut entities = bitvec![0; 32];
        let mut set_bits = 0;

        // Garantir que exatamente `num_entities` bits sejam configurados como 1
        while set_bits < num_entities {
            let index = rng.gen_range(0..4294967295);
            entities.set(index, true);
            set_bits += 1;
        }
    
        entities
    }
}

pub struct BitStringPVS64 {}

impl BitStringPVS64 {
    pub fn update_entities(current_entities: &mut BitArray<u64>, new_entities: &BitArray<u64>) {
        // Assume-se que `current_entities` e `new_entities` têm o mesmo tamanho
        *current_entities = *new_entities;
    }

    // Função para gerar entidades aleatórias
    pub fn generate_random_entities(num_entities: usize) -> BitArray<u64> {
        let mut rng = rand::thread_rng();
        let mut entities: BitArray<u64, Lsb0> = BitArray::ZERO;
        let mut set_bits = 0;

        // Garantir que exatamente `num_entities` bits sejam configurados como 1
        while set_bits < num_entities {
            let index = rng.gen_range(0..18446744073709551615);
            if !entities[index] {
                entities.set(index, true);
                set_bits += 1;
            }
        }
    
        entities
    }
}