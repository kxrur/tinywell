<template>
    <div>
        <Sidebar>
            <SidebarHeader class="border-b border-border">
                <div class="flex items-center gap-2 px-4 py-3">
                    <div class="w-8 h-8 bg-primary rounded-lg flex items-center justify-center mr-6">
                        <Beaker class="w-5 h-5 text-primary-foreground" />
                    </div>
                    <h1 class="text-xl font-bold text-foreground">TinyWell</h1>
                </div>
            </SidebarHeader>

            <SidebarContent class="px-2 py-4">
                <div class="space-y-1">
                    <div v-for="item in presets" :key="item.Name">
                        <SidebarMenuButton
                            class="w-full justify-center py-2 hover:bg-accent hover:text-accent-foreground">
                            <span class="text-center font-medium">{{ item.Name }}</span>
                        </SidebarMenuButton>
                    </div>
                </div>
            </SidebarContent>

            <SidebarFooter class="border-t border-border p-4">
                <Button class="w-full justify-center gap-2" variant="default">
                    <Plus class="w-4 h-4" />
                    <span>Add Preset</span>
                </Button>
            </SidebarFooter>
        </Sidebar>
    </div>
</template>

<script setup lang="ts">
import { Plus, Beaker } from "lucide-vue-next"

interface Preset {
    Name: string
}

const presets = ref<Preset[]>([{ Name: "hehe" }, { Name: "haha" }])
</script>
