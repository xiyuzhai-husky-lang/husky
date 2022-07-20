use crate::*;
use std::fmt::Debug;

impl EntityRoute {
    pub fn root_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.kind {
            EntityRouteKind::Root { ident } => match ident {
                RootIdentifier::Vec => {
                    f.write_str("[]");
                    return self.spatial_arguments[0].take_entity_route().root_fmt(f);
                }
                RootIdentifier::Array => todo!(),
                RootIdentifier::Option => {
                    f.write_str("?");
                    return self.spatial_arguments[0].take_entity_route().root_fmt(f);
                }
                RootIdentifier::Tuple => {
                    f.write_str("(");
                    for (i, spatial_argument) in self.spatial_arguments.iter().enumerate() {
                        if i > 0 {
                            f.write_str(", ")?
                        }
                        spatial_argument.take_entity_route().root_fmt(f)?
                    }
                    return f.write_str(")");
                }
                RootIdentifier::Ref => {
                    f.write_str("&");
                    return self.spatial_arguments[0].take_entity_route().root_fmt(f);
                }
                _ => f.write_str(&ident)?,
            },
            EntityRouteKind::Package { ident, .. } => f.write_str(&ident)?,
            EntityRouteKind::Child { parent, ident } => {
                parent.parent_fmt(f)?;
                f.write_str("::")?;
                f.write_str(&ident)?
            }
            EntityRouteKind::Input { .. } => f.write_str("input")?,
            EntityRouteKind::Generic { ident, .. } => f.write_str(&ident)?,
            EntityRouteKind::ThisType => f.write_str("This")?,
            EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => {
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
        match self.kind {
            EntityRouteKind::Root { ident } => match ident {
                RootIdentifier::Tuple => {
                    f.write_str("(");
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
            EntityRouteKind::Package { ident, .. } => f.write_str(&ident)?,
            EntityRouteKind::Child { parent, ident } => {
                parent.root_fmt(f)?;
                f.write_str("::")?;
                f.write_str(&ident)?
            }
            EntityRouteKind::Input { .. } => f.write_str("input")?,
            EntityRouteKind::Generic { ident, .. } => f.write_str(&ident)?,
            EntityRouteKind::ThisType => f.write_str("This")?,
            EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => {
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
