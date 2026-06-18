<script setup lang="ts">
import { onMounted, ref } from "vue";
import { createPaper, deletePaper, listPapers, type Paper } from "./lib/api";

const papers = ref<Paper[]>([]);
const title = ref("");
const venueName = ref("");
const publicationYear = ref<number | null>(null);
const errorMessage = ref("");

async function refreshPapers() {
  papers.value = await listPapers();
}

async function handleCreatePaper() {
  errorMessage.value = "";

  try {
    await createPaper({
      title: title.value,
      venue_name: venueName.value || null,
      publication_year: publicationYear.value,
      publication_type: "unknown",
    });

    title.value = "";
    venueName.value = "";
    publicationYear.value = null;

    await refreshPapers();
  } catch (error) {
    errorMessage.value = JSON.stringify(error);
  }
}

async function handleDeletePaper(id: string) {
  await deletePaper(id);
  await refreshPapers();
}

onMounted(refreshPapers);
</script>

<template>
  <main class="min-h-screen bg-zinc-950 px-8 py-10 text-zinc-100">
    <section class="mx-auto max-w-5xl">
      <div class="mb-8">
        <p class="text-sm text-zinc-400">Phase 1</p>
        <h1 class="mt-1 text-3xl font-semibold tracking-tight">
          Paper Manager
        </h1>
        <p class="mt-2 text-zinc-400">
          Local SQLite paper metadata CRUD test.
        </p>
      </div>

      <section class="mb-8 rounded-2xl border border-zinc-800 bg-zinc-900 p-6">
        <h2 class="mb-4 text-lg font-medium">Create Paper</h2>

        <div class="grid gap-4 md:grid-cols-3">
          <input
            v-model="title"
            class="rounded-lg border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm outline-none focus:border-zinc-400"
            placeholder="Title"
          />

          <input
            v-model="venueName"
            class="rounded-lg border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm outline-none focus:border-zinc-400"
            placeholder="Venue"
          />

          <input
            v-model.number="publicationYear"
            class="rounded-lg border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm outline-none focus:border-zinc-400"
            placeholder="Year"
            type="number"
          />
        </div>

        <button
          class="mt-4 rounded-lg bg-zinc-100 px-4 py-2 text-sm font-medium text-zinc-950 hover:bg-white"
          @click="handleCreatePaper"
        >
          Add Paper
        </button>

        <p v-if="errorMessage" class="mt-4 text-sm text-red-400">
          {{ errorMessage }}
        </p>
      </section>

      <section class="rounded-2xl border border-zinc-800 bg-zinc-900">
        <div class="border-b border-zinc-800 px-6 py-4">
          <h2 class="text-lg font-medium">Papers</h2>
        </div>

        <table class="w-full text-left text-sm">
          <thead class="text-zinc-400">
            <tr class="border-b border-zinc-800">
              <th class="px-6 py-3 font-medium">Title</th>
              <th class="px-6 py-3 font-medium">Year</th>
              <th class="px-6 py-3 font-medium">Venue</th>
              <th class="px-6 py-3 font-medium">Status</th>
              <th class="px-6 py-3 font-medium"></th>
            </tr>
          </thead>

          <tbody>
            <tr
              v-for="paper in papers"
              :key="paper.id"
              class="border-b border-zinc-800 last:border-0"
            >
              <td class="px-6 py-3">
                {{ paper.title }}
              </td>
              <td class="px-6 py-3 text-zinc-400">
                {{ paper.publication_year ?? "-" }}
              </td>
              <td class="px-6 py-3 text-zinc-400">
                {{ paper.venue_name ?? "-" }}
              </td>
              <td class="px-6 py-3 text-zinc-400">
                {{ paper.reading_status }}
              </td>
              <td class="px-6 py-3 text-right">
                <button
                  class="text-red-400 hover:text-red-300"
                  @click="handleDeletePaper(paper.id)"
                >
                  Delete
                </button>
              </td>
            </tr>

            <tr v-if="papers.length === 0">
              <td class="px-6 py-8 text-center text-zinc-500" colspan="5">
                No papers yet.
              </td>
            </tr>
          </tbody>
        </table>
      </section>
    </section>
  </main>
</template>