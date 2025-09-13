<script setup lang="ts">
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
    </div>
</template>