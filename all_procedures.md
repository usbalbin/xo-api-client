## All remote procedures as reported by [xo-cli](https://github.com/vatesfr/xen-orchestra/tree/master/packages/xo-cli)

Note that only a small subset of those are currently implemented for this library

```
acl.add subject=<string> object=<string> action=<string>
  add a new ACL entry
acl.get
  get existing ACLs
acl.getCurrentPermissions
  get (explicit) permissions by object for the current user
acl.remove subject=<string> object=<string> action=<string>
  remove an existing ACL entry
audit.checkIntegrity [newest=<string>] [oldest=<string>]
  Check records integrity between oldest and newest
audit.clean
  Clean audit database
audit.deleteRange newest=<string> [oldest=<string>]
  Delete a range of records and rewrite the records chain
audit.exportRecords
audit.generateFingerprint [newest=<string>] [oldest=<string>]
  Generate a fingerprint of the chain oldest-newest
audit.getRecords [id=<string>] [ndjson=<boolean>]
  Get records from a passed record ID
backupNg.createJob [compression=<unknown type>] mode=<unknown type> [name=<string>] [proxy=<string>] [remotes=<object>] [schedules=<object>] settings=<object> [srs=<object>] vms=<object>
backupNg.deleteJob id=<string>
backupNg.deleteVmBackup id=<string>
backupNg.editJob [compression=<unknown type>] id=<string> [mode=<unknown type>] [name=<string>] [proxy=<string|null>] [remotes=<object>] [settings=<object>] [srs=<object>] [vms=<object>]
backupNg.fetchFiles disk=<string> [partition=<string>] paths=<array> remote=<string>
backupNg.getAllJobs
backupNg.getAllLogs [ndjson=<boolean>]
backupNg.getJob id=<string>
backupNg.getLogs [after=<number|string>] [before=<number|string>] [limit=<number>] *=<any>
backupNg.getSuggestedExcludedTags
backupNg.importVmBackup id=<string> settings=<object> sr=<string>
backupNg.listFiles disk=<string> [partition=<string>] path=<string> remote=<string>
backupNg.listPartitions disk=<string> remote=<string>
backupNg.listVmBackups [_forceRefresh=<boolean>] remotes=<array>
backupNg.runJob id=<string> schedule=<string> [settings=<object>] [vm=<string>] [vms=<array>]
cloudConfig.create name=<string> template=<string>
  Creates a new cloud config template
cloudConfig.delete id=<string>
  Deletes an existing cloud config template
cloudConfig.getAll
  Gets all existing cloud configs templates
cloudConfig.update id=<string> [name=<string>] [template=<string>]
  Modifies an existing cloud config template
customField.add id=<string> name=<string> value=<string>
  Add a new custom field to an object
customField.remove id=<string> name=<string>
  Remove an existing custom field from an object
customField.set id=<string> name=<string> value=<string>
  Set a custom field
disk.create name=<string> size=<integer|string> sr=<string> [vm=<string>] [bootable=<boolean>] [mode=<string>] [position=<string>]
  create a new disk on a SR
disk.exportContent id=<string>
  export the content of a VDI
disk.import [description=<string>] name=<string> sr=<string> type=<string> [vmdkData=<object>]
disk.importContent id=<string>
  import contents into a VDI
docker.deregister vm=<string>
  Deregister the VM for Docker management
docker.pause vm=<string> container=<string>
docker.register vm=<string>
  Register the VM for Docker management
docker.restart vm=<string> container=<string>
docker.start vm=<string> container=<string>
docker.stop vm=<string> container=<string>
docker.unpause vm=<string> container=<string>
group.addUser id=<string> userId=<string>
  adds a user to a group
group.create name=<string>
  creates a new group
group.delete id=<string>
  deletes an existing group
group.getAll
  returns all the existing group
group.removeUser id=<string> userId=<string>
  removes a user from a group
group.set id=<string> [name=<string>]
  changes the properties of an existing group
group.setUsers id=<string> userIds=<unknown type>
  sets the users belonging to a group
host.detach id=<string>
  eject the host of a pool
host.disable id=<string>
  disable to create VM on the hsot
host.emergencyShutdownHost host=<string>
  suspend all VMs and shutdown host
host.enable id=<string>
  enable to create VM on the host
host.forget id=<string>
  remove the host record from XAPI database
host.getSchedulerGranularity id=<string>
  get the scheduler granularity of a host
host.installCertificate id=<string> certificate=<string> [chain=<string>] privateKey=<string>
  Install a certificate on a host
host.installSupplementalPack host=<string>
  installs supplemental pack from ISO file
host.isHostServerTimeConsistent host=<string>
host.isHyperThreadingEnabled id=<string>
  get hyper-threading information
host.restart id=<string> [force=<boolean>]
  restart the host
host.restartAgent id=<string>
  restart the Xen agent on the host
host.restart_agent id=<string>
  restart the Xen agent on the host
host.scanPifs id=<string>
  Refresh the list of physical interfaces for this host
host.set id=<string> [iscsiIqn=<string>] [name_label=<string>] [name_description=<string>] [multipathing=<boolean>]
  changes the properties of an host
host.setControlDomainMemory id=<string> memory=<number>
  Set host's control domain memory and restart the host
host.setMaintenanceMode id=<string> maintenance=<boolean>
  manage the maintenance mode
host.setRemoteSyslogHost id=<string> syslogDestination=<string>
host.setSchedulerGranularity id=<string> schedulerGranularity=<unknown type>
  set scheduler granularity of a host
host.start id=<string>
  start the host
host.stats host=<string> [granularity=<string>]
  returns statistic of the host
host.stop id=<string>
  stop the host
ipPool.create
  Creates a new ipPool
ipPool.delete
  Delete an ipPool
ipPool.getAll
  List all ipPools
ipPool.set
  Allow to modify an existing ipPool
job.cancel
  Cancel a current run
job.create job=<object>
  Creates a new job from description object
job.delete id=<string>
  Deletes an existing job
job.get id=<string>
  Gets an existing job
job.getAll
  Gets all available jobs
job.runSequence idSequence=<array>
  Runs jobs sequentially, in the provided order
job.set job=<object>
  Modifies an existing job from a description object
log.delete id=<array|string> namespace=<string>
  deletes one or several logs from a namespace
log.get namespace=<string>
  returns logs list for one namespace
message.delete id=<string>
metadataBackup.createJob [name=<string>] [pools=<object>] [proxy=<string>] remotes=<object> schedules=<object> settings=<object> [xoMetadata=<boolean>]
metadataBackup.delete id=<string>
metadataBackup.deleteJob id=<string>
metadataBackup.editJob id=<string> [name=<string>] [pools=<object|null>] [proxy=<string|null>] [settings=<object>] [remotes=<object>] [xoMetadata=<boolean>]
metadataBackup.getAllJobs
metadataBackup.getJob id=<string>
metadataBackup.list remotes=<array>
metadataBackup.restore id=<string>
metadataBackup.runJob id=<string> schedule=<string>
network.create pool=<string> name=<string> [description=<string>] [pif=<string>] [mtu=<integer|string>] [vlan=<integer|string>]
network.createBonded pool=<string> name=<string> [description=<string>] pifs=<array> [mtu=<integer|string>] bondMode=<string>
  Create a bonded network. bondMode can be balance-slb, active-backup or lacp
network.delete id=<string>
network.delete_ id=<string>
network.getBondModes
network.set [automatic=<boolean>] [defaultIsLocked=<boolean>] id=<string> [name_description=<string>] [name_label=<string>]
pbd.connect id=<string>
pbd.delete id=<string>
pbd.disconnect id=<string>
pif.connect id=<string>
pif.delete id=<string>
pif.disconnect id=<string>
pif.editPif id=<string> vlan=<integer|string>
pif.getIpv4ConfigurationModes
pif.getIpv6ConfigurationModes
pif.reconfigureIp [id=<string>] [mode=<string>] [ip=<string>] [netmask=<string>] [gateway=<string>] [dns=<string>]
plugin.configure id=<string> configuration=<unknown type>
  sets the configuration of a plugin
plugin.disableAutoload id=<string>
plugin.enableAutoload id=<string>
  enables a plugin, allowing it to be loaded
plugin.get
  returns a list of all installed plugins
plugin.load id=<string>
  loads a plugin
plugin.purgeConfiguration id=<string>
  removes a plugin configuration
plugin.test id=<string> [data=<unknown type>]
  Test a plugin with its current configuration
plugin.unload id=<string>
  unloads a plugin
plugin.usageReport.send
pool.getLicenseState pool=<string>
pool.getPatchesDifference source=<string> target=<string>
pool.installPatches [pool=<string>] [patches=<array>] [hosts=<array>]
  Install patches on hosts
pool.installSupplementalPack pool=<string>
  installs supplemental pack from ISO file on all hosts
pool.listMissingPatches host=<string>
  return an array of missing new patches in the host
pool.listPoolsMatchingCriteria [minAvailableHostMemory=<number>] [minAvailableSrSize=<number>] [minHostCpus=<number>] [minHostVersion=<string>] [poolNameRegExp=<string>] [srNameRegExp=<string>]
pool.mergeInto [force=<boolean>] [source=<string>] [sources=<array>] target=<string>
pool.patch pool=<string>
pool.rollingUpdate pool=<string>
pool.set id=<string> [name_label=<string>] [name_description=<string>] [migrationNetwork=<string|null>]
pool.setDefaultSr sr=<string>
pool.setPoolMaster host=<string>
pool.uploadPatch pool=<string>
proxy.checkHealth id=<string>
proxy.deploy [httpProxy=<string>] license=<string> sr=<string> [network=<string>] [networkConfiguration=<object>] [proxy=<string>]
proxy.destroy id=<string>
proxy.get id=<string>
proxy.getAll
proxy.getApplianceUpdaterState id=<string>
proxy.register [address=<string>] [vm=<string>] [name=<string>] authenticationToken=<string>
proxy.unregister id=<string>
proxy.update id=<string> [address=<string|null>] [vm=<string|null>] [name=<string>] [authenticationToken=<string>]
proxy.updateApplianceSettings id=<string> [httpProxy=<string|null>]
proxy.upgradeAppliance id=<string> [ignoreRunningJobs=<boolean>]
remote.create name=<string> [options=<string>] [proxy=<string>] url=<string>
  Creates a new fs remote point
remote.delete id=<string>
  Deletes an existing fs remote point
remote.get id=<string>
  Gets an existing fs remote point
remote.getAll
  Gets all existing fs remote points
remote.getAllInfo
  Gets all info of remote
remote.set [enabled=<boolean>] id=<string> [name=<string>] [options=<string|null>] [proxy=<string|null>] [url=<string>]
  Modifies an existing fs remote point
remote.test id=<string>
  Performs a read/write matching test on a remote point
resourceSet.addLimit id=<string> limitId=<string> quantity=<integer>
resourceSet.addObject id=<string> object=<string>
resourceSet.addSubject id=<string> subject=<string>
resourceSet.create name=<string> [subjects=<array>] [objects=<array>] [limits=<object>]
resourceSet.delete id=<string>
resourceSet.get id=<string>
resourceSet.getAll
  Get the list of all existing resource set
resourceSet.recomputeAllLimits
  Recompute manually the current resource set usage
resourceSet.removeLimit id=<string> limitId=<string>
resourceSet.removeObject id=<string> object=<string>
resourceSet.removeSubject id=<string> subject=<string>
resourceSet.set id=<string> [name=<string>] [subjects=<array>] [objects=<array>] [ipPools=<array>] [limits=<object>]
role.getAll
  Returns the list of all existing roles
schedule.create cron=<string> [enabled=<boolean>] jobId=<string> [name=<string>] [timezone=<string>]
  Creates a new schedule
schedule.delete id=<string>
  Deletes an existing schedule
schedule.get id=<string>
  Gets an existing schedule
schedule.getAll
  Gets all existing schedules
schedule.set [cron=<string>] [enabled=<boolean>] id=<string> [jobId=<string>] [name=<string|null>] [timezone=<string>]
  Modifies an existing schedule
server.add [label=<string>] host=<string> username=<string> password=<string> [autoConnect=<boolean>] [allowUnauthorized=<boolean>]
  register a new Xen server
server.disable id=<string>
  disable a Xen server
server.enable id=<string>
  enable a Xen server
server.getAll
  returns all the registered Xen server
server.remove id=<string>
  unregister a Xen server
server.set id=<string> [label=<string>] [host=<string>] [username=<string>] [password=<string>] [allowUnauthorized=<boolean>] [readOnly=<boolean>]
  changes the properties of a Xen server
session.getUser
  return the currently connected user
session.signIn
  sign in
session.signInWithPassword email=<string> password=<string>
  sign in
session.signInWithToken token=<string>
  sign in
session.signOut
  sign out the user from the current session
sr.connectAllPbds id=<string>
sr.createExt host=<string> nameLabel=<string> nameDescription=<string> device=<string>
sr.createHba host=<string> nameLabel=<string> nameDescription=<string> scsiId=<string> [srUuid=<string>]
sr.createIscsi host=<string> nameLabel=<string> nameDescription=<string> target=<string> [port=<integer>] targetIqn=<string> scsiId=<string> [chapUser=<string>] [chapPassword=<string>] [srUuid=<string>]
sr.createIso host=<string> nameLabel=<string> nameDescription=<string> path=<string> type=<string> [user=<string>] [password=<string>] [srUuid=<string>]
sr.createLvm host=<string> nameLabel=<string> nameDescription=<string> device=<string>
sr.createNfs host=<string> nameLabel=<string> nameDescription=<string> server=<string> serverPath=<string> [nfsVersion=<string>] [nfsOptions=<string>] [srUuid=<string>]
sr.createZfs host=<string> nameLabel=<string> nameDescription=<string> location=<string>
sr.destroy id=<string>
sr.disconnectAllPbds id=<string>
sr.forget id=<string>
sr.getUnhealthyVdiChainsLength id=<string>
sr.probeHba host=<string>
sr.probeHbaExists host=<string> scsiId=<string>
sr.probeIscsiExists host=<string> target=<string> [port=<integer>] targetIqn=<string> scsiId=<string> [chapUser=<string>] [chapPassword=<string>]
sr.probeIscsiIqns host=<string> target=<string> [port=<integer>] [chapUser=<string>] [chapPassword=<string>]
sr.probeIscsiLuns host=<string> target=<string> [port=<integer>] targetIqn=<string> [chapUser=<string>] [chapPassword=<string>]
sr.probeNfs host=<string> server=<string>
sr.probeNfsExists host=<string> server=<string> serverPath=<string>
sr.probeZfs host=<string>
sr.scan id=<string>
sr.set id=<string> [name_label=<string>] [name_description=<string>]
sr.stats id=<string> [granularity=<string>]
  returns statistic of the sr
system.getMethodsInfo
  returns the signatures of all available API methods
system.getServerTimezone
  return the timezone server
system.getServerVersion
  return the version of xo-server
system.getVersion
  API version (unstable)
system.listMethods
  returns the name of all available API methods
system.methodSignature
  returns the signature of an API method
tag.add tag=<string> id=<string>
  add a new tag to an object
tag.remove tag=<string> id=<string>
  remove an existing tag from an object
task.cancel id=<string>
task.destroy id=<string>
test.changeConnectedXapiHostname hostname=<string> newObject=<string> oldObject=<string>
  change the connected XAPI hostname and check if the pool and the local cache are updated
test.copyVm vm=<string> sr=<string>
  export/import full/delta VM
test.getPermissionsForUser userId=<string>
test.hasPermission userId=<string> objectId=<string> permission=<string>
test.wait duration=<string>
token.create [expiresIn=<number|string>]
  create a new authentication token
token.delete token=<string>
  delete an existing authentication token
token.deleteAll [except=<string>]
  delete all tokens of the current user except the current one
user.changePassword oldPassword=<string> newPassword=<string>
  change password after checking old password (user function)
user.create email=<string> password=<string> [permission=<string>]
  creates a new user
user.delete id=<string>
  deletes an existing user
user.getAll
  returns all the existing users
user.set id=<string> [email=<string>] [password=<string>] [permission=<string>] [preferences=<object>]
  changes the properties of an existing user
vbd.connect id=<string>
vbd.delete id=<string>
vbd.disconnect id=<string>
vbd.set id=<string> [position=<string|number>]
vbd.setBootable vbd=<string> bootable=<boolean>
vdi.delete id=<string>
vdi.delete_ id=<string>
vdi.migrate id=<string> [resourceSet=<string>] sr_id=<string>
vdi.set id=<string> [name_label=<string>] [name_description=<string>] [size=<integer|string>]
vif.connect id=<string>
vif.delete id=<string>
vif.disconnect id=<string>
vif.getLockingModeValues
vif.set id=<string> [network=<string>] [mac=<string>] [allowedIpv4Addresses=<array>] [allowedIpv6Addresses=<array>] [attached=<boolean>] [lockingMode=<string>] [rateLimit=<number|null>] [resourceSet=<string>] [txChecksumming=<boolean>]
vm.attachDisk [bootable=<boolean>] [mode=<string>] [position=<string>] vdi=<string> vm=<string>
vm.attachPci vm=<string> pciId=<string>
vm.backup id=<string> remoteId=<string> file=<string> [compress=<boolean>]
  Exports a VM to the file system
vm.clone id=<string> name=<string> full_copy=<boolean>
vm.convert id=<string>
vm.convertToTemplate id=<string>
vm.copy [compress=<boolean|string>] [name=<string>] vm=<string> sr=<string>
vm.copyToTemplate id=<string>
vm.create [affinityHost=<string>] [bootAfterCreate=<boolean>] [cloudConfig=<string>] [networkConfig=<string>] [coreOs=<boolean>] [clone=<boolean>] [coresPerSocket=<string|number>] [resourceSet=<string>] [installation=<object>] [vgpuType=<string>] [gpuGroup=<string>] name_label=<string> [name_description=<string>] [pv_args=<string>] [share=<boolean>] template=<string> [VIFs=<array>] [VDIs=<array>] [existingDisks=<object>] [hvmBootFirmware=<string>] [copyHostBiosStrings=<boolean>] *=<any>
vm.createCloudInitConfigDrive vm=<string> sr=<string> config=<string> [networkConfig=<string>]
vm.createInterface vm=<string> network=<string> [position=<integer|string>] [mac=<string>] [allowedIpv4Addresses=<array>] [allowedIpv6Addresses=<array>]
vm.createVgpu vm=<string> gpuGroup=<string> vgpuType=<string>
vm.delete id=<string> [deleteDisks=<boolean>] [force=<boolean>] [forceDeleteDefaultTemplate=<boolean>]
vm.deleteVgpu vgpu=<string>
vm.deltaCopy [force=<boolean>] id=<string> [retention=<number>] sr=<string>
vm.detachPci vm=<string>
vm.ejectCd id=<string>
vm.export vm=<string> [compress=<boolean|string>]
vm.getCloudInitConfig template=<string>
vm.getHaValues
vm.import [data=<object>] [type=<string>] sr=<string>
vm.importBackup remote=<string> file=<string> sr=<string>
  Imports a VM into host, from a file found in the chosen remote
vm.importDeltaBackup sr=<string> remote=<string> filePath=<string> [mapVdisSrs=<object>]
vm.insertCd id=<string> cd_id=<string> [force=<boolean>]
vm.migrate vm=<string> [force=<boolean>] targetHost=<string> [sr=<string>] [mapVdisSrs=<object>] [mapVifsNetworks=<object>] [migrationNetwork=<string>]
vm.pause id=<string>
vm.recoveryStart id=<string>
vm.restart id=<string> [force=<boolean>]
vm.resume id=<string>
vm.revert snapshot=<string>
vm.rollingBackup id=<string> remoteId=<string> tag=<string> [retention=<number>] [depth=<number>] [compress=<boolean>]
  Exports a VM to the file system with a tagged name, and removes the oldest backup with the same tag according to retention
vm.rollingDeltaBackup id=<string> remote=<string> tag=<string> [retention=<string|number>] [depth=<string|number>]
vm.rollingDrCopy [retention=<number>] [depth=<number>] id=<string> [pool=<string>] [sr=<string>] tag=<string> [deleteOldBackupsFirst=<boolean>]
  Copies a VM to a different pool, with a tagged name, and removes the oldest VM with the same tag from this pool, according to retention
vm.rollingSnapshot id=<string> tag=<string> [retention=<number>] [depth=<number>]
  Snapshots a VM with a tagged name, and removes the oldest snapshot with the same tag according to retention
vm.set id=<string> [auto_poweron=<boolean>] [name_label=<string>] [name_description=<string>] [high_availability=<string>] [CPUs=<integer>] [cpusMax=<integer|string>] [memory=<integer|string>] [memoryMin=<integer|string>] [memoryMax=<integer|string>] [memoryStaticMax=<integer|string>] [PV_args=<string>] [cpuMask=<array>] [cpuWeight=<integer|null>] [cpuCap=<integer|null>] [affinityHost=<string|null>] [vga=<string>] [videoram=<number>] [coresPerSocket=<string|number|null>] [hasVendorDevice=<boolean>] [expNestedHvm=<boolean>] [resourceSet=<string|null>] [share=<boolean>] [startDelay=<integer>] [secureBoot=<boolean>] [nicType=<string|null>] [hvmBootFirmware=<string|null>] [virtualizationMode=<string>] [blockedOperations=<object>]
vm.setBootOrder vm=<string> order=<string>
vm.snapshot [description=<string>] id=<string> [name=<string>] [saveMemory=<boolean>]
vm.start [bypassMacAddressesCheck=<boolean>] [force=<boolean>] [host=<string>] id=<string>
vm.stats id=<string> [granularity=<string>]
  returns statistics about the VM
vm.stop id=<string> [force=<boolean>]
vm.suspend id=<string>
xo.clean
xo.exportConfig [entries=<array>] [passphrase=<string>]
xo.getAllObjects [filter=<object>] [limit=<number>] [ndjson=<boolean>]
  Returns all XO objects
xo.importConfig [passphrase=<string>]
xosan.addBricks xosansr=<string> lvmsrs=<array> brickSize=<number>
  add brick to XOSAN SR
xosan.checkSrCurrentState poolId=<string>
  checks if there is an operation currently running on the SR
xosan.computeXosanPossibleOptions lvmSrs=<array> [brickSize=<number>]
xosan.createSR [brickSize=<number>] srs=<array> template=<object> pif=<string> vlan=<string> glusterType=<string> redundancy=<number> [memorySize=<number>] [ipRange=<string>]
  create gluster VM
xosan.downloadAndInstallXosanPack id=<string> version=<string> pool=<string>
  Register a resource via cloud plugin
xosan.fixHostNotInNetwork xosanSr=<string> host=<string>
  put host in xosan network
xosan.getVolumeInfo sr=<string> infoType=<string>
  info on gluster volume
xosan.profileStatus sr=<string> [changeStatus=<bool>]
  activate, deactivate, or interrogate profile data
xosan.removeBricks xosansr=<string> bricks=<array>
  remove brick from XOSAN SR
xosan.replaceBrick xosansr=<string> previousBrick=<string> newLvmSr=<string> brickSize=<number>
  replaceBrick brick in gluster volume
xosan.unlock licenseId=<string> sr=<string>
  Unlock XOSAN SR functionalities by binding it to a paid license
```
