export const translations = {
	en: {
		meta: { label: 'English' },
		common: {
			back: 'Back to Profiles',
			cancel: 'Cancel',
			saving: 'Saving...'
		},
		home: {
			addButton: '+ Add Profile',
			edit: 'Edit',
			exclusionCount: '{count} exclusions',
			title: 'Game Profiles',
			empty: 'No profiles found.',
			subtitle: 'Manage synchronization rules and paths',
			sync: 'Sync'
		},
		form: {
			add: 'Add',
			addButton: 'Create Profile',
			added: 'Profile added',
			confirmDelete: 'Delete',
			delete: 'Delete Profile',
			deleted: 'Profile deleted',
			destinationLabel: 'Destination Path',
			editButton: 'Save Changes',
			exclusionsDeleteLabel: 'Protected Paths in Delete',
			exclusionsSyncLabel: 'Protected Paths In Sync',
			exclusionsPlaceholder: 'logs or log.txt (name not path)',
			failedAdd: 'Failed to add profile',
			failedDelete: 'Failed to delete profile',
			failedUpdate: 'Failed to update profile',
			messageDelete: 'Are you sure you want to delete ',
			nameLabel: 'Profile Name',
			sourceLabel: 'Source Path',
			titleCreate: 'Create Profile',
			titleDelete: 'Delete Profile',
			titleEdit: 'Edit Profile',
			updated: 'Profile updated'
		},
		navbar: {
			profiles: 'Profiles'
		},
		settings: {
			failed: 'Failed to save settings',
			jobsLabel: 'Parallel Jobs',
			jobsSubtitle:
				'Number of parallel operations. Defaults to 75% of available CPU cores, but higher values can be tried at your own risk of slowing down the computer.',
			save: 'Save Settings',
			saved: 'Settings saved',
			subtitle: 'Manage global settings',
			title: 'Settings'
		},
		sync: {
			dryRun: 'Dry Run',
			dryRunActive: 'Dry Run Active',
			failed: 'Failed to sync profile',
			logs: 'Logs',
			logsPlaceholder: 'Sync output will appear here...',
			showSkipped: 'Show Skipped Files',
			simulate: 'Simulate',
			simulating: 'Simulating...',
			sync: 'Sync Mods',
			synced: 'Profile synced successfully',
			syncing: 'Syncing...',
			title: 'Sync Profile'
		}
	},

	'pt-BR': {
		meta: { label: 'Português' },
		common: {
			back: 'Voltar para Perfis',
			cancel: 'Cancelar',
			saving: 'Salvando...'
		},
		home: {
			addButton: '+ Adicionar Perfil',
			edit: 'Alterar',
			exclusionCount: '{count} exclusões',
			title: 'Perfis de Jogos',
			empty: 'Nenhum perfil encontrado.',
			subtitle: 'Gerencie regras de sincronização e caminhos',
			sync: 'Sincronizar'
		},
		form: {
			add: 'Adicionar',
			addButton: 'Criar Perfil',
			added: 'Perfil criado',
			confirmDelete: 'Excluir',
			delete: 'Excluir Perfil',
			deleted: 'Perfil excluído',
			destinationLabel: 'Caminho de Destino',
			editButton: 'Salvar Alterações',
			exclusionsDeleteLabel: 'Caminhos Protegidos na Exclusão',
			exclusionsSyncLabel: 'Caminhos Protegidos na Sincronização',
			exclusionsPlaceholder: 'logs ou log.txt (nome não caminho)',
			failedAdd: 'Falha ao criar perfil',
			failedDelete: 'Falha ao excluir perfil',
			failedUpdate: 'Falha ao atualizar perfil',
			messageDelete: 'Tem certeza que deseja excluir ',
			nameLabel: 'Nome do Perfil',
			sourceLabel: 'Caminho de Origem',
			titleCreate: 'Criar Perfil',
			titleDelete: 'Excluir Perfil',
			titleEdit: 'Editar Perfil',
			updated: 'Perfil atualizado'
		},
		navbar: {
			profiles: 'Perfis'
		},
		settings: {
			failed: 'Falha ao salvar as configurações',
			jobsLabel: 'Threads Paralelos',
			jobsSubtitle:
				'Número de threads paralelas. Por padrão é 75% dos nucleos disponíveis, mas valores mais altos podem ser tentados a seu próprio risco de reduzir a performance do computador.',
			save: 'Salvar Configurações',
			saved: 'Configurações salvas',
			subtitle: 'Gerencie configurações globais',
			title: 'Configurações'
		},
		sync: {
			dryRun: 'Simulação',
			dryRunActive: 'Simulação Ativa',
			failed: 'Falha ao sincronizar perfil',
			logs: 'Logs',
			logsPlaceholder: 'Os logs da sincronização aparecerão aqui...',
			showSkipped: 'Mostrar Arquivos Ignorados',
			simulate: 'Simular',
			simulating: 'Simulando...',
			sync: 'Sincronizar Mods',
			synced: 'Perfil sincronizado com sucesso',
			syncing: 'Sincronizando...',
			title: 'Sincronizar Perfil'
		}
	}
} as const;

type TranslatableStructure = Omit<typeof translations.en, 'meta'>;

type NestedKeys<T> = T extends object
	? {
			[K in keyof T & string]: T[K] extends object ? `${K}.${NestedKeys<T[K]>}` : K;
		}[keyof T & string]
	: never;

export type TranslationKey = NestedKeys<TranslatableStructure> | (string & {});
