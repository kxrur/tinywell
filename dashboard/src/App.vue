<script setup lang="ts">
import { Toaster } from '@/components/ui/sonner'
</script>

<template>
  <div class="w-full h-screen">
    <SidebarProvider>
      <div class="flex w-full h-full">
        <NavBar />
        <main class="flex-1 w-full">
          <div class="w-full">
            <SidebarTrigger />
            <MainWindow />
          </div>
        </main>
      </div>
    </SidebarProvider>
    <Toaster close-button rich-colors />
  </div>
</template>
