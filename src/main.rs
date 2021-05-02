use std::process::Command;
use serde_json;
use serde::{Deserialize, Serialize};

fn main() {
    let instances = get_instances();


    for instance in instances.iter() {
        let msg = format!("Instace ID: {} || Instance Type: {} || Instance Public IP: {}", instance.InstanceId, instance.InstanceType, instance.PublicIpAddress);
        println!("{}", &msg); // Example of simplicity with structs
    }
}

fn get_instances() -> Vec<Instance> {
    let mut instances: Vec<Instance> = Vec::new();
    let command = Command::new("aws")
        .arg("ec2")
        .arg("describe-instances")
        .output()
        .expect("Failed to get ec2 instances.");

    let output = String::from_utf8_lossy(&command.stdout);
    let ec2_reservations: EC2 = serde_json::from_str(&output)
    .expect("Failed to create EC2 object...");
    for reservation in ec2_reservations.Reservations.iter() {
        for instance in reservation.Instances.iter() {
            let owned = instance.clone();
            instances.push(owned);
        }
    }
    return instances;
}

fn get_instances_with_tag(key: &str, value: &str) -> Vec<Instance> {
    let instances = get_instances();
    let mut output: Vec<Instance> = Vec::new();
    for instance in instances.iter() {
        for tag in instance.Tags.iter(){
            if tag.Key == key {
                if tag.Value == value {
                    output.push(instance.clone());
                }
            }
        }
    }
    return output;
}


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct EC2 {
    Reservations: Vec<Instances>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Instances {
    Instances: Vec<Instance>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Instance {
    AmiLaunchIndex: i32,
    ImageId: String,
    InstanceId: String,
    InstanceType: String,
    KeyName: String,
    LaunchTime: String,
    PublicDnsName: String,
    PublicIpAddress: String,
    Tags: Vec<Tag>,
    Monitoring: Monitoring,
    Placement: Placement,
    PrivateDnsName: String,
    PrivateIpAddress: String,
    State: State,
    StateTransitionReason: String,
    SubnetId: String,
    VpcId: String,
    Architecture: String,
    BlockDeviceMappings: Vec<BlockDevice>,
    ClientToken: String,
    EbsOptimized: bool,
    EnaSupport: bool,
    Hypervisor: String,
    NetworkInterfaces: Vec<NetworkInterface>,
    RootDeviceName: String,
    RootDeviceType: String,
    SecurityGroups: Vec<Group>,
    SourceDestCheck: bool

}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct NetworkInterface {
    Association: Association,
    Attachment: Attachment,
    Description: String,
    Groups: Vec<Group>,
    MacAddress: String,
    NetworkInterfaceId: String,
    OwnerId: String,
    PrivateDnsName: String,
    PrivateIpAddress: String,
    PrivateIpAddresses: Vec<PrivateIpAddress>,
    SourceDestCheck: bool,
    Status: String,
    SubnetId: String,
    VpcId: String,
    InterfaceType: String
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct PrivateIpAddress {
    Association: Association,
    Primary: bool,
    PrivateDnsName: String,
    PrivateIpAddress: String
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Attachment {
    AttachTime: String,
    AttachmentId: String,
    DeleteOnTermination: bool,
    DeviceIndex: i32,
    Status: String,
    NetworkCardIndex: u32
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Group {
    GroupName: String,
    GroupId: String
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Association {
    IpOwnerId: String,
    PublicDnsName: String,
    PublicIp: String
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Ebs {
    AttachTime: String,
    DeleteOnTermination: bool,
    Status: String,
    VolumeId: String
}


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct BlockDevice {
    DeviceName: String,
    Ebs: Ebs
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Placement {
    AvailabilityZone: String,
    GroupName: String,
    Tenancy: String
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct State {
    Code: i32,
    Name: String
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Tag {
    Key: String,
    Value: String
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Monitoring {
    State: String
}