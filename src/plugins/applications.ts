import { invoke } from '@tauri-apps/api/tauri'
import { IItemGroup } from '../components/ItemGroup.vue';
import { IListItem } from '../components/ListItem.vue';

export default class ApplicationsPlugin {
    public static async getItemGroup(): Promise<IItemGroup> {

        
        const output = await invoke('get_desktop_files');
        console.log(output);
    
    
    
        const applications: IListItem[] = [
    
        ];
    
        return {
            name: "Apps",
            icon: "rocket",
            items: applications,
        }
    }
}

