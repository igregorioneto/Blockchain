use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
    program_error::ProgramError, msg, program_pack::Pack, sysvar::Sysvar, system_instruction,
};

struct MyContract {
    value: u64,
}

// Define uma conta estado para o contrato
solana_program::declare_id!("MyContract1111111111111111111111111111111111111");

// Função para inicializar o contrato
fn init_contract(program_id: &Pubkey, accounts: &[AccountInfo], initial_value: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let my_contract_account = next_account_info(accounts_iter)?;
    let payer_account = next_account_info(accounts_iter)?;

    // Verifica se o contrato já foi inicializado
    if my_contract_account.data_is_empty() {
        // Cria o estado inicial do contrato
        let data = MyContract { value: initial_value };
        data.pack_into_slice(&mut my_contract_account.data.borrow_mut())?;
    } else {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Associa o contrato ao programa
    let seeds = &[b"MyContract"];
    let signer_seeds = &[&seeds[..]];

    // Transfere lamports para a conta do contrato (opcional)
    let lamports_to_transfer = 10; // Quantidade de lamports a transferir
    if lamports_to_transfer > 0 {
        let transfer_instruction = system_instruction::transfer(
            payer_account.key,
            my_contract_account.key,
            lamports_to_transfer,
        );
        msg!("Transferring {} lamports to the contract.", lamports_to_transfer);
        solana_program::program::invoke_signed(
            &transfer_instruction,
            accounts,
            signer_seeds,
        )?;
    }

    Ok(())
}

// Função para registrar um novo valor no contrato
fn register_value(program_id: &Pubkey, accounts: &[AccountInfo], new_value: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let my_contract_account = next_account_info(accounts_iter)?;

    // Carrega o estado atual do contrato
    let mut data = MyContract::unpack(&my_contract_account.data.borrow())?;
    data.value = new_value;

    // Atualiza o estado do contrato
    data.pack_into_slice(&mut my_contract_account.data.borrow_mut())?;

    Ok(())
}

// Ponto de entrada do programa
entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    match instruction_data[0] {
        0 => {
            // Inicializa o contrato
            let initial_value = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            init_contract(program_id, accounts, initial_value)
        }
        1 => {
    // Registra um novo valor no contrato
            let new_value = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            register_value(program_id, accounts, new_value)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

// Função auxiliar para obter a próxima conta da lista
fn next_account_info<'a, 'b>(
    iter: &mut std::slice::Iter<'a, AccountInfo<'b>>,
) -> Result<AccountInfo<'a>, ProgramError> {
    iter.next().ok_or(ProgramError::NotEnoughAccountKeys)
}

// Implementação do deserializador do estado do contrato
impl<'a> solana_program::program_pack::IsInitialized for MyContract {
    fn is_initialized(&self) -> bool {
        true
    }
}

// Implementação do deserializador do estado do contrato
impl<'a> solana_program::program_pack::Pack for MyContract {
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let value = u64::from_le_bytes(src[0..8].try_into().unwrap());
        Ok(MyContract { value })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) -> Result<(), ProgramError> {
        dst.copy_from_slice(&self.value.to_le_bytes());
        Ok(())
    }
}

