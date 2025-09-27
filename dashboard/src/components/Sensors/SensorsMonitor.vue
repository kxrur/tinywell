<template>
    <div class="flex flex-col space-y-4 p-4 bg-background">
        <div class="flex items-center justify-between">
            <Label class="text-lg font-semibold">Sensors Monitor</Label>
        </div>

        <div class="border rounded-lg">
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead class="w-[200px]">Sensor</TableHead>
                        <TableHead>Current Value</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow>
                        <TableCell class="font-medium">
                            <div class="flex items-center space-x-2">
                                <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                                <span>Temperature (TEMP-01)</span>
                            </div>
                        </TableCell>
                        <TableCell>
                            <Badge variant="outline">{{ temperature.toFixed(1) }}°C</Badge>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell class="font-medium">
                            <div class="flex items-center space-x-2">
                                <div class="w-3 h-3 bg-blue-500 rounded-full"></div>
                                <span>Humidity (HUM-01)</span>
                            </div>
                        </TableCell>
                        <TableCell>
                            <Badge variant="outline">{{ humidity.toFixed(1) }}%</Badge>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell class="font-medium">
                            <div class="flex items-center space-x-2">
                                <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                                <span>Pressure (PRES-01)</span>
                            </div>
                        </TableCell>
                        <TableCell>
                            <Badge variant="outline">{{ pressure.toFixed(0) }} hPa</Badge>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell class="font-medium">
                            <div class="flex items-center space-x-2">
                                <div class="w-3 h-3 bg-amber-500 rounded-full"></div>
                                <span>Light (LIGHT-01)</span>
                            </div>
                        </TableCell>
                        <TableCell>
                            <Badge variant="outline">{{ light.toFixed(0) }} lux</Badge>
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </div>
    </div>
</template>

<script setup lang="ts">

const temperature = ref(23.5) // °C
const humidity = ref(65.2)    // %
const pressure = ref(1013.2)  // hPa
const light = ref(450)        // lux
const lastUpdated = ref('')


// Functions
const simulateReadings = () => {
    temperature.value = 15 + Math.random() * 35 // 15-50°C
    humidity.value = 30 + Math.random() * 70    // 30-100%
    pressure.value = 950 + Math.random() * 150  // 950-1100 hPa
    light.value = Math.random() * 1000          // 0-1000 lux
    updateTimestamp()
}

const updateTimestamp = () => {
    const now = new Date()
    lastUpdated.value = now.toLocaleTimeString()
}

// Initialize
onMounted(() => {
    simulateReadings()
})
</script>
