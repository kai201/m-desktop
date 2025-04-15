import axios from 'axios';
import axiosTauriApiAdapter from 'axios-tauri-api-adapter';
const client = axios.create({ adapter: axiosTauriApiAdapter });