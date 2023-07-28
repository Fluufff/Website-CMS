import { Badge, Button, ButtonLink, ButtonSizes, ButtonTypes, ITableColumn } from '@ibs/components';
import { CONTENT_TYPE_KINDS_TRANSLATIONS, ContentTypeKinds, IField } from '@ibs/shared';
import { TFunction } from 'i18next';
import * as yup from 'yup';

export const addContentComponentSchema = yup.object({
	contentComponentId: yup.string().required(),
	name: yup.string().required(),
});

export const CONTENT_LIST_COLUMNS = (t: TFunction): ITableColumn[] => [
	{
		id: 'name',
		label: 'Name',
	},
	{
		id: 'slug',
		label: 'Slug',
	},
	{
		id: 'kind',
		label: 'Kind',
		format: (value) => <Badge>{CONTENT_TYPE_KINDS_TRANSLATIONS[value as ContentTypeKinds]}</Badge>,
	},
	{
		id: 'fields',
		label: 'Content Components',
		format: (value) => ((value as IField[]) || []).length,
	},
	{
		id: 'actions',
		label: '',
		format: (value, key, item) => (
			<div className="u-display-flex">
				<ButtonLink to={`${item.id}`} size={ButtonSizes.SMALL} type={ButtonTypes.SECONDARY} className="u-margin-left-auto">
					<i className="las la-pen"></i> Edit
				</ButtonLink>
				<Button size={ButtonSizes.SMALL} type={ButtonTypes.SECONDARY} className="u-margin-left-sm">
					<i className="las la-trash"></i>
				</Button>
			</div>
		),
	},
];
