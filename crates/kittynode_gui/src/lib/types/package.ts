export interface Package {
  description: string;
  network_name: string;
  containers: Container[];
}

export interface Container {
  name: string;
  image: string;
  cmd: string[];
  port_bindings: Record<string, { host_ip: string; host_port: string }[]>;
  volume_bindings: string[];
}
