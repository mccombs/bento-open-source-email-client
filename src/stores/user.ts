import { defineStore } from 'pinia';
import { supabase } from '@/lib/supabase';
import log from "@/lib/logger";


export default defineStore('user', {
    state: () => ({
        user: null as object | null,
    }),
    actions: {
        async login(email: string) {
            const { error } = await supabase.auth.signInWithOtp({ email });
            if (error) throw error;
        //   this.user = user;
        },
        async logout() {
            const { error } = await supabase.auth.signOut();
            if (error) throw error;
            this.user = null;
        },
        async fetchUser() {
            const user = await supabase.auth.getUser();
            console.log(user.data)
            if (user.data !== null) {
                log(user.data);
                this.user = user.data;
            }
        }
    }
});
