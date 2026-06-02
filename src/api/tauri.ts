import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

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

export interface LogEvent {
  level: string;
  message: string;
}

/// 打开文件选择对话框
export async function openFileDialog(): Promise<string | null> {
  return invoke<string | null>("open_file_dialog");
}

/// 打开保存文件对话框
export async function saveFileDialog(): Promise<string | null> {
  return invoke<string | null>("save_file_dialog");
}

/// 读取 Excel 文件元数据
export async function readFileMeta(filePath: string): Promise<FileMeta> {
  return invoke<FileMeta>("read_file_meta", { filePath });
}

/// 处理加班数据
export async function processData(request: ProcessRequest): Promise<ProcessResult> {
  return invoke<ProcessResult>("process_data", { request });
}

/// 监听日志事件
export async function onLogEvent(callback: (event: LogEvent) => void) {
  return listen<LogEvent>("log", (e) => {
    callback(e.payload);
  });
}
