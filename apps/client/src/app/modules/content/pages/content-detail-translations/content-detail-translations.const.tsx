import { Button, ButtonLink, ButtonSizes, ButtonTypes, ITableColumn } from '@ibs/components';
import { TFunction } from 'i18next';
import { generatePath } from 'react-router-dom';
import * as yup from 'yup';
import { IContentType } from '@ibs/shared';

import { CONTENT_PATHS } from '../../content.routes';

export const addContentComponentSchema = yup.object({
	contentComponentId: yup.string().required(),
	name: yup.string().required(),
});

export const CONTENT_LIST_COLUMNS = (contentType: IContentType, t: TFunction): ITableColumn[] => [
	{
		id: 'language.key',
		label: 'Language',
		// width: '150px',
		format: (value) => (value as string).toUpperCase()
	},
	{
		id: 'name',
		label: 'Name',
		format: (value) => value as string || <i className='u-text--light'>No content item</i>
	},
	// {
	// 	id: 'published',
	// 	label: 'Published',
	// 	format(value, key, item, index) {
	// 		if (value === true) {
	// 			return <span className='las la-check u-text--success'></span>
	// 		}

	// 		if (value === false) {
	// 			return <span className='las la-times u-text--danger'></span>;
	// 		}
	// 	},
	// },
	{
		id: 'actions',
		label: '',
		// width: '150px',
		format: (value, key, item) => {
			if (item.id) {
				return <div className="u-display-flex">
					<ButtonLink to={generatePath(CONTENT_PATHS.DETAIL, { contentId: item.id, kind: contentType.kind.toLowerCase() })} size={ButtonSizes.SMALL} type={ButtonTypes.SECONDARY} className="u-margin-left-auto">
						<i className="las la-pen"></i> Edit
					</ButtonLink>
				</div>
			}

			return <div className="u-display-flex">
				<ButtonLink to={`${generatePath(CONTENT_PATHS.CREATE_DETAIL, { contentTypeId: contentType.id, kind: contentType.kind.toLowerCase() })}?translationId=${item.translationId}&languageId=${(item.language as any).id}`} size={ButtonSizes.SMALL} type={ButtonTypes.SECONDARY} className="u-margin-left-auto">
					<i className="las la-plus"></i> Create
				</ButtonLink>
			</div>
		}
	},
];
