import axiosInstance from "@/utils/http";
import { fetch } from "@tauri-apps/plugin-http";

export async function getUploadUrl() {
    let rs = await axiosInstance.get("https://whelp.ares-ai.cn/msg/uptoken", { params: { fileName: 'mhepl/' + Date.now() + '.jpg' } });

    return rs.data.data;
}

export function Upload(url: string, file: any) {
    // return axiosInstance.put(url, file);
    return fetch(url, {
        method: 'PUT',
        body: file
    });
}