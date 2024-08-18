<template>
  <div :class="cn('grid gap-6', $attrs.class ?? '')">
    <!-- <form @submit="handleLogin">
            <div class="grid gap-2">
                <div class="grid gap-1">
                    <Label class="sr-only" for="email">
                        Email
                    </Label>
                    <Input id="email" placeholder="name@example.com" type="email" v-model="email" auto-capitalize="none"
                        auto-complete="email" auto-correct="off" :disabled="isLoading" />
                </div>
                <Button :disabled="isLoading">
                    <LucideSpinner v-if="isLoading" class="mr-2 h-4 w-4 animate-spin" />
                    Sign In with Email
                </Button>
            </div>
        </form>
        <div class="relative">
            <div class="absolute inset-0 flex items-center">
                <span class="w-full border-t" />
            </div>
            <div class="relative flex justify-center text-xs uppercase">
                <span class="bg-background px-2 text-muted-foreground">
                    Or continue with
                </span>
            </div>
        </div> -->
    <Button
      @click="handleLogin"
      variant="outline"
      type="button"
      :disabled="isLoading">
      Google
    </Button>
  </div>
</template>

<script setup lang="ts">
import { supabase } from "@/lib/supabase";
import { ref } from "vue";

const handleLogin = async (event: any) => {
  console.log("handleLogin");
  // const loader = await loadingController.create({})
  // const toast = await toastController.create({ duration: 5000 })
  // e.preventDefault()
  isLoading.value = true;
  event.preventDefault();

  try {
    // await loader.present()
    // console.log(await supabase.auth.signIn({ email: email.value }))
    const { data, error } = await supabase.auth.signInWithOAuth({
      provider: "google",
      options: {
        redirectTo: "http://localhost:1420/",
        scopes: "https://mail.google.com",
      },
    });

    console.log(data, error);

    if (error) throw error;

    supabase.auth.onAuthStateChange((event, session) => {
      if (session && session.provider_token) {
        window.localStorage.setItem(
          "oauth_provider_token",
          session.provider_token
        );
      }

      if (session && session.provider_refresh_token) {
        window.localStorage.setItem(
          "oauth_provider_refresh_token",
          session.provider_refresh_token
        );
      }

      if (event === "SIGNED_OUT") {
        window.localStorage.removeItem("oauth_provider_token");
        window.localStorage.removeItem("oauth_provider_refresh_token");
      }
    });
    // toast.message = 'Check your email for the login link!'
    // await toast.present()
  } catch (error: any) {
    // toast.message = error.error_description || error.message
    // await toast.present()
  } finally {
    // await loader.dismiss()
  }
};

import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";

const isLoading = ref(false);
// async function onSubmit(event: Event) {
//   event.preventDefault()
//   isLoading.value = true

//   setTimeout(() => {
//     isLoading.value = false
//   }, 3000)
// }
</script>
