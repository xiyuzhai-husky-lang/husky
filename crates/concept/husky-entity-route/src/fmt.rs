use crate::*;
use std::fmt::Debug;

impl EntityRoute {
    pub fn root_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.variant {
            EntityRouteVariant::Root { ident } => match ident {
                RootBuiltinIdentifier::Vec => {
                    if self.spatial_arguments.len() > 0 {
                        f.write_str("[]")?;
                        return self.entity_route_argument(0).root_fmt(f);
                    } else {
                        f.write_str("Vec")?
                    }
                }
                RootBuiltinIdentifier::Array => todo!(),
                RootBuiltinIdentifier::Option => {
                    f.write_str("?")?;
                    return self.entity_route_argument(0).root_fmt(f);
                }
                RootBuiltinIdentifier::Tuple => {
                    f.write_str("(")?;
                    for (i, spatial_argument) in self.spatial_arguments.iter().enumerate() {
                        if i > 0 {
                            f.write_str(", ")?
                        }
                        spatial_argument.take_entity_route().root_fmt(f)?
                    }
                    return f.write_str(")");
                }
                RootBuiltinIdentifier::Ref => {
                    f.write_str("&")?;
                    return self.entity_route_argument(0).root_fmt(f);
                }
                _ => f.write_str(&ident)?,
            },
            EntityRouteVariant::Package { ident, .. } => f.write_str(&ident)?,
            EntityRouteVariant::Child { parent, ident } => {
                parent.parent_fmt(f)?;
                f.write_str("::")?;
                f.write_str(&ident)?
            }
            EntityRouteVariant::TargetInputValue { .. } => f.write_str("input")?,
            EntityRouteVariant::TargetOutputType { .. } => f.write_str("TargetOutput")?,
            EntityRouteVariant::Any { ident, .. } => f.write_str(&ident)?,
            EntityRouteVariant::ThisType { .. } => f.write_str("This")?,
            EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => {
                f.write_str("<")?;
                ty.root_fmt(f)?;
                f.write_str(" as ")?;
                trai.root_fmt(f)?;
                f.write_str(">::")?;
                ident.fmt(f)?
            }
        };
        if self.spatial_arguments.len() > 0 {
            f.write_str("<")?;
            for (i, generic) in self.spatial_arguments.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }
                match generic {
                    SpatialArgument::Const(_) => todo!(),
                    SpatialArgument::EntityRoute(scope) => scope.root_fmt(f)?,
                }
            }
            f.write_str(">")?;
        }
        Ok(())
    }
    pub fn parent_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.variant {
            EntityRouteVariant::Root { ident } => match ident {
                RootBuiltinIdentifier::Tuple => {
                    f.write_str("(")?;
                    for (i, spatial_argument) in self.spatial_arguments.iter().enumerate() {
                        if i > 0 {
                            f.write_str(", ")?
                        }
                        spatial_argument.take_entity_route().root_fmt(f)?
                    }
                    return f.write_str(")");
                }
                _ => f.write_str(&ident)?,
            },
            EntityRouteVariant::Package { ident, .. } => f.write_str(&ident)?,
            EntityRouteVariant::Child { parent, ident } => {
                parent.root_fmt(f)?;
                f.write_str("::")?;
                f.write_str(&ident)?
            }
            EntityRouteVariant::TargetInputValue { .. } => f.write_str("input")?,
            EntityRouteVariant::TargetOutputType { .. } => f.write_str("TargetOutput")?,
            EntityRouteVariant::Any { ident, .. } => f.write_str(&ident)?,
            EntityRouteVariant::ThisType { .. } => f.write_str("This")?,
            EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => {
                f.write_str("<")?;
                ty.root_fmt(f)?;
                f.write_str(" as ")?;
                trai.root_fmt(f)?;
                f.write_str(">::")?;
                ident.fmt(f)?
            }
        };
        if self.spatial_arguments.len() > 0 {
            f.write_str("<")?;
            for (i, generic) in self.spatial_arguments.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }
                match generic {
                    SpatialArgument::Const(_) => todo!(),
                    SpatialArgument::EntityRoute(scope) => scope.root_fmt(f)?,
                }
            }
            f.write_str(">")?;
        }
        Ok(())
    }
}
