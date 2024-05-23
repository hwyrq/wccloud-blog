<template>
  <el-space style="" direction="vertical">
    <el-card  shadow="hover" style="width: 400px;height: 220px;"  >
      <div class="cloud" v-if="labelData.length > 0">
        <h2 >标签云</h2>
          <el-link style="margin: 5px;color: white;padding: 5px;"
                   v-for="item in labelData" :key="item.labelId"   >
            {{item.labelName}}
          </el-link>
      </div>
    </el-card>
  </el-space>

</template>

<script setup lang="ts">

import {useAsyncData} from "#app";
import {label} from "~/api";

let {data: {value: data}} = await useAsyncData(() => label());
const labelData = data.data;

</script>

<style scoped lang="css">
.cloud  a:nth-child(8n-7) { background: rgba(138, 155, 15, 0.85)
}
.cloud  a:nth-child(8n-6) { background: rgba(235, 104, 65, 0.85)
}
.cloud  a:nth-child(8n-5) { background: rgba(63, 184, 175, 0.85)
}
.cloud  a:nth-child(8n-4) { background: rgba(254, 67, 101, 0.85)
}
.cloud  a:nth-child(8n-3) { background: rgba(252, 157, 154, 0.85)
}
.cloud  a:nth-child(8n-2) { background: rgba(237, 201, 81, 0.85)
}
.cloud  a:nth-child(8n-1) { background: rgba(200, 200, 169, 0.85)
}
.cloud  a:nth-child(8n) { background: rgba(131, 175, 155, 0.85)
}
.cloud  a:first-child { background: rgba(3, 101, 100, 0.85)
}
.cloud  a:last-child { background: rgba(50, 153, 187, 0.85)
}
.cloud  a:hover { border-radius: 0; text-shadow: #000 1px 1px 1px }
</style>
