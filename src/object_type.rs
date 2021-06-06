use jsonrpsee_ws_client::JsonValue;

/// Object type
///
/// This is most often used with [`crate::Client::get_objects_of_type`] to specify what type
/// of objects to fetch
///
/// NOTE: This is only checked by running
/// `xo-cli --list-objects --type | grep type | sort | uniq`
/// on an existing XO-setup. Thus there may be more types
// TODO: Check if there are more types
#[derive(Debug)]
pub enum ObjectType {
    GpuGroup,

    /// A virualization host, likely XCP-ng or similar
    Host,
    Message,
    Network,

    Pbd,
    Pci,

    /// Physical graphics card
    Pgpu,

    /// Physical network interface of a host
    Pif,

    /// Pool of hosts
    Pool,

    /// Storage repository - Place where the disks of VMs are stored ([`Self::Vdi`]s)
    Sr,

    Task,
    Vbd,

    /// Virtual disk, the disks of virtual machines
    Vdi,
    VdiSnapshot,
    VdiUnmanaged,

    /// Virtual network interface of a VM
    Vif,

    /// A virtual machine
    Vm,
    VmController,

    /// Snapshot of a VM
    VmSnapshot,

    /// Virtual machine template, used to easily create preconfigured VMs
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
