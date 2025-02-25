export interface SysInfo {
  cpus: CpuInfo[];
  memory: MemoryInfo;
}

export interface CpuInfo {
  brand: string;
  cpu_usage: number;
  frequency: number;
  name: string;
  vendor_id: string;
}

export interface MemoryInfo {
  total_memory: number;
  used_memory: number;
}
