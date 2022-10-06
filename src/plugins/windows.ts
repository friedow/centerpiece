import { invoke } from '@tauri-apps/api/tauri'
import { IItemGroup } from '../components/ItemGroup.vue';

export default class WindowsPlugin {
    public isLoading = true;
    public itemGroup: IItemGroup | null = null;

    public getItemGroup(): IItemGroup | null {
        return this.itemGroup;
    }

    public initialize() {
        this.loadData();
        setInterval(() => this.loadData(), 3000);
    }

    public async loadData() {
        this.itemGroup = await invoke('get_windows_group');
        this.isLoading = false;
    }
}
