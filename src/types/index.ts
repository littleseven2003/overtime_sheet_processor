export interface FileMeta {
  file_path: string;
  file_name: string;
  labs: string[];
  total_rows: number;
}

export interface ProcessRequest {
  file_path: string;
  selected_labs: string[];
  output_path: string;
}

export interface ProcessResult {
  success: boolean;
  output_path: string;
  total_records: number;
  message: string;
}

export interface LogEntry {
  level: "info" | "success" | "error";
  message: string;
  time: string;
}
