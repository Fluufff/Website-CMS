import { useEffect } from 'react';
import { generatePath, useParams } from 'react-router-dom';
import { FormProvider, useForm } from 'react-hook-form';
import { yupResolver } from '@hookform/resolvers/yup';

import { Alert, AlertTypes, Button, ButtonTypes, HTMLButtonTypes, RenderFields } from '~components';

import { CONTENT_COMPONENT_PATHS } from '../../content-components.routes';
import { editFieldSchema } from '../cc-field-detail/cc-field-detail.const';

import { useContentComponentFieldStore, useContentComponentStore, useHeaderStore } from '~shared';

interface IEditFieldForm {
	name: string;
	config: Record<string, unknown>;
	min: number;
	max: number;
	multiLanguage: boolean;
}

export const CCFieldConfigurationPage = () => {
	const [contentComponent] = useContentComponentStore((state) => [state.contentComponent]);
	const [contentComponentField] = useContentComponentFieldStore((state) => [state.field]);
	const { siteId } = useParams();
	const [setBreadcrumbs] = useHeaderStore((state) => [state.setBreadcrumbs]);
	const formMethods = useForm<IEditFieldForm>({
		resolver: yupResolver(editFieldSchema),
		values: contentComponentField,
	});
	const [updateField, updateFieldLoading] = useContentComponentFieldStore((state) => [state.updateField, state.updateFieldLoading]);

	const {
		handleSubmit,
		formState: { errors },
	} = formMethods;

	useEffect(() => {
		setBreadcrumbs([
			{ label: 'Content Components', to: generatePath(CONTENT_COMPONENT_PATHS.ROOT, { siteId }) },
			{
				label: contentComponent?.name,
				to: generatePath(CONTENT_COMPONENT_PATHS.DETAIL, {
					contentComponentId: contentComponent?.id || '',
					siteId,
				}),
			},
			{
				label: 'Content Components',
				to: generatePath(CONTENT_COMPONENT_PATHS.DETAIL_CC, {
					contentComponentId: contentComponent?.id || '',
					siteId,
				}),
			},
			{
				label: contentComponentField?.name,
				to: generatePath(CONTENT_COMPONENT_PATHS.FIELD_DETAIL, {
					contentComponentId: contentComponent?.id || '',
					fieldId: contentComponentField?.id || '',
					siteId,
				}),
			},
			{
				label: 'Configuration',
			},
		]);
	}, [contentComponent, contentComponentField]);

	const onSubmit = (values: IEditFieldForm) => {
		if (!contentComponent || !contentComponentField) {
			return;
		}

		updateField(siteId!, contentComponent.id!, contentComponentField?.id, values);
	};

	return (
		<FormProvider {...formMethods}>
			<form onSubmit={handleSubmit(onSubmit)}>
				<Alert className="u-margin-bottom" type={AlertTypes.DANGER}>
					{errors?.root?.message}
				</Alert>
				<RenderFields siteId={siteId!} fieldPrefix="config." fields={contentComponentField?.contentComponent?.configurationFields || []} />

				<div className="u-margin-top">
					<Button type={ButtonTypes.PRIMARY} htmlType={HTMLButtonTypes.SUBMIT}>
						{updateFieldLoading && <i className="las la-redo-alt la-spin"></i>} Save
					</Button>
				</div>
			</form>
		</FormProvider>
	);
};
