export interface SysInfo {
  cpus: CpuInfo[];
}

export interface CpuInfo {
  brand: string;
  cpu_usage: number;
  frequency: number;
  name: string;
  vendor_id: string;
}
