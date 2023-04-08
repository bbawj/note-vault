import { App, Modal, normalizePath, OpenViewState, PaneType, Plugin, PluginSettingTab, Pos, prepareSimpleSearch, renderResults, SearchResult, setIcon, Setting, SplitDirection, TFile, WorkspaceLeaf } from 'obsidian';

import * as plugin from "./pkg/obsidian_rust_plugin.js";
import * as wasmbin from './pkg/obsidian_rust_plugin_bg.wasm';

interface semanticSearchSettings {
	apiKey: string;
}

const DEFAULT_SETTINGS: semanticSearchSettings = {
	apiKey: ''
}

export default class SemanticSearch extends Plugin {
	settings: semanticSearchSettings;

	async onload() {
		await this.loadSettings();

		this.addRibbonIcon('file-search-2', 'Semantic Search', (evt: MouseEvent) => {
      new QueryModal(this.app, this.settings).open();
		});

		this.addCommand({
			id: 'open-query-modal',
			name: 'Open query modal',
			callback: () => {
				new QueryModal(this.app, this.settings).open();
			}
		});

		this.addSettingTab(new SettingTab(this.app, this));

		// here's the Rust bit
		await plugin.default(Promise.resolve(wasmbin.default));
		plugin.onload(this);
	}

	onunload() {

	}

	async loadSettings() {
		this.settings = Object.assign({}, DEFAULT_SETTINGS, await this.loadData());
	}

	async saveSettings() {
		await this.saveData(this.settings);
	}
}

type WASMSuggestion = {
  name: string
  header: string
}

class Suggestion {
  app: App;
  name: string;
  header: string;
  pos: Pos | undefined;
  file: TFile | undefined;
  match: SearchResult | undefined;

  constructor(app: App, wasmSuggestion: WASMSuggestion) {
    this.app = app;
    this.name = wasmSuggestion.name;
    this.header = wasmSuggestion.header;
  }

  // Find corresponding suggestion file
  addSuggestionFile() : Suggestion {
    const files = this.app.vault.getMarkdownFiles();
    const matching_file = files.find(file => file.name === this.name);
    this.file = matching_file;
    return this;
  }

  addSuggestionHeading() {
    const { metadataCache } = this.app;
    if (this.file) {
      const headingList = metadataCache.getFileCache(this.file)?.headings ?? [];
      const search = prepareSimpleSearch(this.header);
      headingList.forEach(heading => {
        if (heading.heading === this.header) {
          this.pos = heading.position;
          const match = search(heading.heading);
          if (match) {
            this.match = match;
          }
        }
      })
    }
    return this;
  }
}

export class QueryModal extends Modal {
  settings: semanticSearchSettings = DEFAULT_SETTINGS;
  estimatedCost = 0;

  constructor(app: App, settings: semanticSearchSettings) {
    super(app);
    this.settings = settings;
  }

  onOpen(): void {
      const contentEl = this.contentEl;

      const inputContainer = contentEl.createDiv({cls: "prompt-input-container"})
      const input = inputContainer.createEl("input", {cls: "prompt-input"});

      const div = contentEl.createDiv()
      div.createSpan({text: "Estimated cost of query: " + this.estimatedCost});

      const button = div.createEl("button", {text: "Submit"});
      const resultsDiv = contentEl.createDiv({cls: "prompt-results"});
      button.onclick = async () => {
        const suggestions: Suggestion[] = await this.getSuggestions(input.value);
        suggestions.forEach(suggestion => {
          const resultContainer = resultsDiv.createDiv({cls: ["suggestion-item", "mod-complex"]})
          this.renderSuggestion(suggestion, resultContainer);
        })
      }
  }

  onClose() {
    let { contentEl } = this;
    contentEl.empty();
  }

  // Returns all available suggestions.
  async getSuggestions(query: string): Promise<Suggestion[]> {
    const wasmSuggestions: WASMSuggestion[] = await plugin.get_suggestions(this.app, this.settings.apiKey, query);
    const suggestions: Suggestion[] = wasmSuggestions.map(wasmSuggestion => new Suggestion(this.app, wasmSuggestion));

    suggestions.forEach(suggestion => {
      suggestion.addSuggestionFile().addSuggestionHeading();
    })

    return suggestions;
  }

  // Renders each suggestion item.
  renderSuggestion(suggestion: Suggestion, el: HTMLElement) {
    // const div = el.createEl("div", { text: suggestion.name});
    // el.createEl("small", { text: suggestion.header});
    el.onclick = async () => await this.onChooseSuggestion(suggestion);
    if (suggestion.match && suggestion.file) {
      const div = this.renderContent(el, suggestion.header, suggestion.match);
      this.renderPath(div, suggestion.file, suggestion.match);
    }
  }

  renderContent(
    parentEl: HTMLElement,
    content: string,
    match: SearchResult,
    offset?: number,
  ): HTMLDivElement {
    const contentEl = parentEl.createDiv({
      cls: ['suggestion-content', 'qsp-content'],
    });

    const titleEl = contentEl.createDiv({
      cls: ['suggestion-title', 'qsp-title'],
    });

    renderResults(titleEl, content, match, offset);

    return contentEl;
  }

  renderPath(
    parentEl: HTMLElement,
    file: TFile,
    match: SearchResult,
  ): void {
    if (parentEl && file) {
      const isRoot = file.parent.isRoot();
      let hidePath = isRoot;

      if (!hidePath) {
        const wrapperEl = parentEl.createDiv({ cls: ['suggestion-note', 'qsp-note'] });
        const path = this.getPathDisplayText(file);

        const iconEl = wrapperEl.createSpan({ cls: ['qsp-path-indicator'] });
        setIcon(iconEl, 'folder');

        const pathEl = wrapperEl.createSpan({ cls: 'qsp-path' });
        renderResults(pathEl, path, match);
      }
    }
  }

  getPathDisplayText(
    file: TFile,
  ): string {
    let text = '';

    if (file) {
      const { parent } = file;
      const dirname = parent.name;
      const isRoot = parent.isRoot();
      text = isRoot ? `${file.name}` : normalizePath(`${dirname}/${file.name}`);
    }

    return text;
  }

  // Perform action on the selected suggestion.
  async onChooseSuggestion(suggestion: Suggestion) {
    const isMatch = (candidateLeaf: WorkspaceLeaf) => {
      let val = false;

      if (candidateLeaf?.view) {
        val = candidateLeaf.view.file === suggestion.file;
      }

      return val;
    };
    const leaves: WorkspaceLeaf[] = [];
    this.app.workspace.iterateAllLeaves(leaf => leaves.push(leaf));
    const matchingLeaf = leaves.find(isMatch);

    const eState = {
      startLoc: suggestion.pos?.start,
      endLoc: suggestion.pos?.end,
    }

    if (matchingLeaf === undefined) {
      if (suggestion.file) {
        await this.openFileInLeaf(suggestion.file, "tab", "vertical", {
          active: true,
          eState
        })
      }
    } else {
      this.app.workspace.setActiveLeaf(matchingLeaf, {focus: true});
      matchingLeaf.view.setEphemeralState(eState);
    }

    this.close();
  }



  async openFileInLeaf(file: TFile, navType: PaneType, splitDirection: SplitDirection = "vertical", openState: OpenViewState) {
    const { workspace } = this.app;
    const leaf = navType === "split" ? workspace.getLeaf(navType, splitDirection) : workspace.getLeaf(navType)
    await leaf.openFile(file, openState);
  }
}

class SettingTab extends PluginSettingTab {
	plugin: SemanticSearch;

	constructor(app: App, plugin: SemanticSearch) {
		super(app, plugin);
		this.plugin = plugin;
	}

	display(): void {
		const {containerEl} = this;

		containerEl.empty();

		containerEl.createEl('h2', {text: 'Obsidian Semantic Search'});

		new Setting(containerEl)
			.setName('OpenAI API Key')
			.setDesc('https://platform.openai.com/account/api-keys')
			.addText(text => text
				.setPlaceholder('Enter your secret')
				.setValue(this.plugin.settings.apiKey)
				.onChange(async (value) => {
					this.plugin.settings.apiKey = value;
					await this.plugin.saveSettings();
				}));
	}
}
