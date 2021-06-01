use jsonrpsee_ws_client::JsonValue;

/// NOTE: This is only checked by running
/// `xo-cli --list-objects --type | grep type | sort | uniq`
/// on an existing XO-setup. Thus there may be more types
// TODO: Check if there are more types
pub enum ObjectType {
    GpuGroup,
    Host,
    Message,
    Network,
    Pbd,
    Pci,
    Pgpu,
    Pif,
    Pool,
    Sr,
    Task,
    Vbd,
    Vdi,
    VdiSnapshot,
    VdiUnmanaged,
    Vif,
    Vm,
    VmController,
    VmSnapshot,
    VmTemplate,
}

impl ToString for ObjectType {
    fn to_string(&self) -> String {
        match self {
            ObjectType::GpuGroup => "gpuGroup",
            ObjectType::Host => "host",
            ObjectType::Message => "message",
            ObjectType::Network => "network",
            ObjectType::Pbd => "PBD",
            ObjectType::Pci => "PCI",
            ObjectType::Pgpu => "PGPU",
            ObjectType::Pif => "PIF",
            ObjectType::Pool => "pool",
            ObjectType::Sr => "SR",
            ObjectType::Task => "task",
            ObjectType::Vbd => "VBD",
            ObjectType::Vdi => "VDI",
            ObjectType::VdiSnapshot => "VDI-snapshot",
            ObjectType::VdiUnmanaged => "VDI-unmanaged",
            ObjectType::Vif => "VIF",
            ObjectType::Vm => "VM",
            ObjectType::VmController => "VM-controller",
            ObjectType::VmSnapshot => "VM-snapshot",
            ObjectType::VmTemplate => "VM-template",
        }
        .to_string()
    }
}

impl From<ObjectType> for JsonValue {
    fn from(val: ObjectType) -> Self {
        JsonValue::String(val.to_string())
    }
}
