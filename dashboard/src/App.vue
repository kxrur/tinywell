<script setup lang="ts">
</script>

<template>
    <div>
        <SidebarProvider>
            <NavBar></NavBar>
            <main>
                <SidebarTrigger />
                <MainWindow></MainWindow>
            </main>
        </SidebarProvider>
    </div>
</template>