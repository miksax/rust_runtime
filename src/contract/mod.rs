use crate::{blockchain::AddressHash, mem::WaBuffer, types::CallData};

pub mod op_20;

pub trait ContractTrait {
    fn set_environment(&mut self, environment: &'static crate::blockchain::Environment);

    fn environment(&self) -> &'static crate::blockchain::Environment;

    fn is_self(&self, address: &AddressHash) -> bool {
        address.eq(&self.environment().address)
    }

    fn only_deployer(&self, caller: &AddressHash) -> Result<(), crate::error::Error> {
        if self.environment().deployer.ne(caller) {
            Err(crate::error::Error::OnlyOwner)
        } else {
            Ok(())
        }
    }

    fn emit(event: &impl crate::event::EventTrait) -> Result<(), crate::error::Error> {
        crate::env::emit(event.buffer());
        Ok(())
    }

    fn on_deploy(&mut self, _call_data: CallData) {
        crate::log("On Deploy is not implemented");
    }

    fn execute(&mut self, _call_data: CallData) -> Result<WaBuffer, crate::error::Error> {
        crate::log("Execute is not implemented");
        unimplemented!("Execute needs to be implemented");
    }
}
