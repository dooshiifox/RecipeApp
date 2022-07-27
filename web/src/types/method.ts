import type { Formattable } from '.';

export interface Method {
	steps: Step[];
}

export interface Step {
	title: string;
	substeps: SubStep[];
}

export interface SubStep {
	content: Formattable;
	image?: string;
	warnings?: Warning[];
	infos?: Info[];
}

export interface Warning {
	title: string;
	content: Formattable;
}

export interface Info {
	title: string;
	content: Formattable;
}
