import { App, Editor, fuzzySearch, MarkdownView, Modal, Notice, OpenViewState, PaneType, Plugin, PluginSettingTab, Pos, prepareQuery, renderResults, SearchResult, Setting, SplitDirection, SuggestModal, TFile, WorkspaceLeaf } from 'obsidian';

import * as plugin from "./pkg/obsidian_rust_plugin.js";
import * as wasmbin from './pkg/obsidian_rust_plugin_bg.wasm';

interface semanticSearchSettings {
	apiKey: string;
}

const DEFAULT_SETTINGS: semanticSearchSettings = {
	apiKey: ''
}

export default class MyPlugin extends Plugin {
	settings: semanticSearchSettings;

	async onload() {
		await this.loadSettings();

		// This creates an icon in the left ribbon.
		const ribbonIconEl = this.addRibbonIcon('dice', 'Sample Plugin', (evt: MouseEvent) => {
			// Called when the user clicks the icon.
			new Notice('This is a notice!');
		});
		// Perform additional things with the ribbon
		ribbonIconEl.addClass('my-plugin-ribbon-class');

		// This adds a status bar item to the bottom of the app. Does not work on mobile apps.
		const statusBarItemEl = this.addStatusBarItem();
		statusBarItemEl.setText('Status Bar Text');

		// This adds a simple command that can be triggered anywhere
		this.addCommand({
			id: 'open-sample-modal-simple',
			name: 'Open sample modal (simple)',
			callback: () => {
				new QueryModal(this.app, this.settings).open();
			}
		});
		// This adds an editor command that can perform some operation on the current editor instance
		this.addCommand({
			id: 'sample-editor-command',
			name: 'Sample editor command',
			editorCallback: (editor: Editor, view: MarkdownView) => {
				console.log(editor.getSelection());
				editor.replaceSelection('Sample Editor Command');
			}
		});

		// This adds a settings tab so the user can configure various aspects of the plugin
		this.addSettingTab(new SettingTab(this.app, this));

		// If the plugin hooks up any global DOM events (on parts of the app that doesn't belong to this plugin)
		// Using this function will automatically remove the event listener when this plugin is disabled.
		this.registerDomEvent(document, 'click', (evt: MouseEvent) => {
			console.log('click', evt);
		});

		// When registering intervals, this function will automatically clear the interval when the plugin is disabled.
		this.registerInterval(window.setInterval(() => console.log('setInterval'), 5 * 60 * 1000));

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
    const prepQuery = prepareQuery(this.header);
    const { metadataCache } = this.app;
    if (this.file) {
      const headingList = metadataCache.getFileCache(this.file)?.headings ?? [];
      headingList.forEach(heading => {
        const match = fuzzySearch(prepQuery, heading.heading);
        if (match) {
          this.pos = heading.position;
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
      contentEl.createEl("h1", {text: "Enter your search query"});
      const input = contentEl.createEl("input", {cls: "query_input"});
      const div = contentEl.createDiv()
      div.createSpan({text: "Estimated cost of query: " + this.estimatedCost});
      const button = div.createEl("button", {text: "Submit"});
      button.onclick = async () => {
        const suggestions: Suggestion[] = await this.getSuggestions(input.value);
        suggestions.forEach(suggestion => {
          this.renderSuggestion(suggestion, contentEl);
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
      const headingCache = this.findSuggestionHeading(suggestion, matching_file);
    })
    return suggestions;
  }

  // Renders each suggestion item.
  renderSuggestion(suggestion: Suggestion, el: HTMLElement) {
    const div = el.createEl("div", { text: suggestion.name});
    el.createEl("small", { text: suggestion.header});
    div.onclick = async () => await this.onChooseSuggestion(suggestion);
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

    if (matchingLeaf === undefined && suggestion.file) {
      await this.openFileInLeaf(suggestion.file, "tab", "vertical", {
        active: true,
        eState
      })
    }
  }



  async openFileInLeaf(file: TFile, navType: PaneType, splitDirection: SplitDirection = "vertical", openState: OpenViewState) {
    const { workspace } = this.app;
    const leaf = navType === "split" ? workspace.getLeaf(navType, splitDirection) : workspace.getLeaf(navType)
    await leaf.openFile(file, openState);
  }
}

class SettingTab extends PluginSettingTab {
	plugin: MyPlugin;

	constructor(app: App, plugin: MyPlugin) {
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
